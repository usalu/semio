"""🌊 jlstress: graph-free stress harness for the change-journal watcher.

Runs one watcher binary (`semio-repo-watch watch ...`) on a private root, drives file churn the way the fleet does
(clone bursts like `mirror.py create`, creations in rooted-excluded, name-excluded and watched directories) and
counts what the watcher reports: rows, `lost` frames by reason, and SILENT MISSES (a created watched path that is
neither reported nor covered by a `lost` frame). Streams everything, keeps RSS small, writes one JSON summary.

Usage: python3 -I jlstress.py --binary B --root R --out OUT.json [--nice 15] [--n 16000] [--phases clone,excluded,named,included]
       [--slow-us 0] [--hog 0] [--latency-ms 5] [--label L] [--rounds 1]
"""
import argparse, ctypes, json, os, resource, shutil, subprocess, sys, threading, time

LIBC = ctypes.CDLL("libc.dylib" if sys.platform == "darwin" else None, use_errno=True)


def rss_guard():
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > (500 << 20 if sys.platform == "darwin" else 500 << 10):
        sys.exit("[DEBUG] jlstress above 500 MB, abort")


def tree(base, n, width=100):
    for i in range(n):
        d = os.path.join(base, f"d{i // width:04d}")
        if i % width == 0:
            os.makedirs(d, exist_ok=True)
        yield os.path.join(d, f"f{i:05d}.txt")


def clone(src, dst):
    if sys.platform == "darwin":
        if LIBC.clonefile(src.encode(), dst.encode(), 0) != 0:
            raise OSError(ctypes.get_errno(), "clonefile " + src)
    else:
        shutil.copyfile(src, dst)


class Watcher:
    def __init__(self, a, root):
        cmd = [a.binary, "watch", "--root", root, "--latency-ms", str(a.latency_ms), "--exclude", "ex", "--exclude-name", "node_modules"]
        if a.nice:
            cmd = ["nice", "-n", str(a.nice)] + cmd
        self.p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=open(a.stderr, "ab") if a.stderr else subprocess.DEVNULL, bufsize=1 << 16)
        self.ready_frame = None
        self.slow = a.slow_us / 1e6
        self.ready = threading.Event()
        self.paths, self.subtrees, self.lost, self.rows, self.batches, self.frames = set(), set(), [], 0, 0, {}
        self.lock = threading.Lock()
        threading.Thread(target=self.read, daemon=True).start()

    def read(self):
        for line in self.p.stdout:
            frame = json.loads(line)
            kind = frame.get("t")
            with self.lock:
                self.frames[kind] = self.frames.get(kind, 0) + 1
                if kind == "ready":
                    self.ready_frame = frame
                    self.ready.set()
                elif kind == "batch":
                    self.batches += 1
                    self.rows += len(frame["c"])
                    for row in frame["c"]:
                        self.paths.add(row[2])
                        if row[1] == 1 and row[0] != "m":
                            self.subtrees.add(row[2])
                elif kind == "lost":
                    self.lost.append([round(time.time(), 3), frame.get("reason"), frame.get("scope")])
            if self.slow:
                time.sleep(self.slow)

    def covered(self, path):
        if path in self.paths:
            return True
        parts = path.split("/")
        return any("/".join(parts[:i]) in self.subtrees for i in range(1, len(parts)))

    def fence(self, root, name, timeout=60):
        open(os.path.join(root, name), "w").close()
        deadline = time.time() + timeout
        while time.time() < deadline:
            with self.lock:
                if name in self.paths:
                    return round(timeout - (deadline - time.time()), 3)
            time.sleep(0.01)
        return None

    def stop(self):
        try:
            self.p.stdin.write(b"stop\n"); self.p.stdin.flush()
        except OSError:
            pass
        try:
            self.p.wait(5)
        except subprocess.TimeoutExpired:
            self.p.kill()


def replay_history(binary, root, ready, timeout=60):
    p = subprocess.Popen([binary, "watch", "--root", root, "--since", ready["cursor"], "--uuid", ready["uuid"], "--latency-ms", "5"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=open(os.environ.get("JL_REPLAY_ERR", os.devnull), "ab"))
    paths, deadline = set(), time.time() + timeout
    for line in p.stdout:
        frame = json.loads(line)
        if frame.get("t") == "batch":
            paths.update(r[2] for r in frame["c"])
        if frame.get("t") == "history-done" or time.time() > deadline:
            break
    p.kill(); p.wait()
    return paths


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", required=True); ap.add_argument("--root", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--nice", type=int, default=15); ap.add_argument("--n", type=int, default=16000); ap.add_argument("--latency-ms", type=int, default=5)
    ap.add_argument("--phases", default="clone,excluded,named,included"); ap.add_argument("--slow-us", type=int, default=0)
    ap.add_argument("--hog", type=int, default=0); ap.add_argument("--label", default="run"); ap.add_argument("--rounds", type=int, default=1)
    ap.add_argument("--stderr", default=""); ap.add_argument("--history", action="store_true")
    a = ap.parse_args()
    root = os.path.realpath(a.root)
    if os.path.exists(root):
        shutil.rmtree(root)
    os.makedirs(os.path.join(root, "src")); os.makedirs(os.path.join(root, "ex")); os.makedirs(os.path.join(root, "in")); os.makedirs(os.path.join(root, "nm", "node_modules"))
    for r in range(a.rounds):
        for i in range(0, a.n, 100):
            os.makedirs(os.path.join(root, "in", f"r{r}", f"d{i // 100:04d}"))
    sources = []
    for path in tree(os.path.join(root, "src"), a.n):
        with open(path, "w") as h:
            h.write("x")
        sources.append(os.path.relpath(path, root))
    outside = root + ".clones"
    shutil.rmtree(outside, ignore_errors=True)
    time.sleep(2)
    hogs = [subprocess.Popen([sys.executable, "-I", "-c", "while True: pass"]) for _ in range(a.hog)]
    w = Watcher(a, root)
    started = time.time()
    if not w.ready.wait(20):
        sys.exit("[DEBUG] watcher not ready")
    phases, missed, created_all = [], 0, []
    try:
        for r in range(a.rounds):
            for phase in a.phases.split(","):
                rss_guard()
                with w.lock:
                    lost0, rows0 = len(w.lost), w.rows
                t0 = time.time()
                created = []
                if phase == "clone":
                    os.makedirs(outside + f"/{r}", exist_ok=True)
                    for i, rel in enumerate(sources):
                        d = os.path.join(outside, str(r), os.path.dirname(rel))
                        if i % 100 == 0:
                            os.makedirs(d, exist_ok=True)
                        clone(os.path.join(root, rel), os.path.join(outside, str(r), rel))
                elif phase in ("excluded", "named", "included"):
                    base = {"excluded": "ex", "named": "nm/node_modules", "included": "in"}[phase] + f"/r{r}"
                    for path in tree(os.path.join(root, base), a.n):
                        with open(path, "w") as h:
                            h.write("y")
                        if phase == "included":
                            created.append(os.path.relpath(path, root))
                took = time.time() - t0
                fence = w.fence(root, f"fence-{r}-{phase}")
                with w.lock:
                    lost = w.lost[lost0:]
                    rows = w.rows - rows0
                    silent = sum(1 for p in created if not w.covered(p)) if created else 0
                created_all.extend(created)
                if silent and not lost:
                    missed += silent
                phases.append({"round": r, "phase": phase, "files": a.n, "writeS": round(took, 2), "fenceS": fence, "rows": rows, "lost": len(lost), "lostReasons": sorted({l[1] for l in lost}), "unreported": silent, "silentMiss": silent if not lost else 0})
                print("[DEBUG]", json.dumps(phases[-1]), flush=True)
        if a.history:
            unreported = [p for p in created_all if not w.covered(p)]
            history = replay_history(a.binary, root, w.ready_frame)
            phases.append({"phase": "history", "unreported": len(unreported), "inHistory": sum(1 for p in unreported if p in history), "historyPaths": len(history)})
            print("[DEBUG]", json.dumps(phases[-1]), flush=True)
    finally:
        w.stop()
        for h in hogs:
            h.kill()
        shutil.rmtree(outside, ignore_errors=True)
        shutil.rmtree(root, ignore_errors=True)
    load = os.getloadavg()[0]
    summary = {"label": a.label, "binary": a.binary, "nice": a.nice, "latencyMs": a.latency_ms, "slowUs": a.slow_us, "hog": a.hog, "load1": round(load, 1), "seconds": round(time.time() - started, 1), "frames": w.frames, "lostTotal": len(w.lost), "silentMiss": missed, "phases": phases}
    with open(a.out, "a") as h:
        h.write(json.dumps(summary) + "\n")
    print(json.dumps({k: summary[k] for k in ("label", "nice", "load1", "lostTotal", "silentMiss", "frames")}))


if __name__ == "__main__":
    main()
