#!/usr/bin/env python3
"""🧪 Language-agnostic law of the native watcher: while the consumer does not read stdout, a burst of file creations must not make the watcher lose events.
usage: python3 -I law-stall.py WATCH_BINARY [--files N] [--stall SECONDS] [--expect-loss]
The oracle is the file system itself: every created file name must arrive in a batch frame and no `lost` frame may appear. `--expect-loss` inverts the verdict (the law is red on the base watcher)."""
import json, os, shutil, subprocess, sys, tempfile, time
def arg(name, default):
    return type(default)(sys.argv[sys.argv.index(name) + 1]) if name in sys.argv else default
def main():
    binary, files, stall = sys.argv[1], arg("--files", 8000), arg("--stall", 3.0)
    if sys.platform.startswith("win"):
        print("SKIP law-stall: Windows"); return 0
    root = os.path.realpath(tempfile.mkdtemp(prefix="watchlaw-"))
    proc = subprocess.Popen([binary, "watch", "--root", root, "--latency-ms", "5"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    try:
        assert b'"ready"' in proc.stdout.readline()
        made = set()
        for directory in range(files // 100):
            base = os.path.join(root, "d%d" % directory, "x" * 60)
            os.makedirs(base)
            for index in range(100):
                name = "f%03d-%s.txt" % (index, "y" * 40)
                with open(os.path.join(base, name), "w") as handle: handle.write("a")
                made.add("d%d/%s/%s" % (directory, "x" * 60, name))
        time.sleep(stall)
        os.set_blocking(proc.stdout.fileno(), False)
        deadline, buffer, lines = time.time() + 8, b"", []
        while time.time() < deadline:
            try: chunk = os.read(proc.stdout.fileno(), 1 << 20)
            except BlockingIOError: chunk = None
            if chunk: buffer += chunk; deadline = max(deadline, time.time() + 1.5)
            else: time.sleep(0.05)
            *done, buffer = buffer.split(b"\n")
            lines += done
        frames = [json.loads(line) for line in lines if line.strip()]
        seen = {row[2] for frame in frames if frame["t"] == "batch" for row in frame["c"]}
        lost = [frame for frame in frames if frame["t"] == "lost"]
        missing = len(made - seen)
        bad = bool(lost) or missing > 0
        print(json.dumps({"files": files, "stall": stall, "seen": len(made & seen), "missing": missing, "lost": [(f["reason"], f.get("scope")) for f in lost][:5]}))
        verdict = bad if "--expect-loss" in sys.argv else not bad
        print("LAW " + ("PASS" if verdict else "FAIL"))
        return 0 if verdict else 1
    finally:
        try: proc.stdin.write(b"stop\n"); proc.stdin.flush(); proc.wait(5)
        except Exception: proc.kill()
        shutil.rmtree(root, ignore_errors=True)
sys.exit(main())
