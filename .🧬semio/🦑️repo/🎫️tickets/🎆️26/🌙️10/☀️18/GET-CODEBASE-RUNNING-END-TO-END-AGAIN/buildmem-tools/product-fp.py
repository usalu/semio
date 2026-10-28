"""📐 [buildmem] product.py + physical footprint: `product-fp.py <label> <target>` writes `🗑️generated/buildmem/product/<label>.{log,json}`; next to the ps RSS tree it samples `top -l 1 -stats pid,ppid,mem` (macOS physical footprint, which excludes the freed-but-reclaimable pages that ps RSS keeps counting) and records the five processes with the largest footprint (pid, peak RSS, peak footprint, command).

Samples the process-tree RSS every 5 s, kills only its own process group above 20 GB or after 45 min of running time (time parked in the capacity or graph queue does not count) and records wall, graph rows and exit."""
import json, os, re, signal, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
OUT = os.path.join(HERE, "..", "🗑️generated", "buildmem", "product")
UNITS = {"B": 1 / 1024, "K": 1, "M": 1024, "G": 1024 * 1024}
LIMIT_KB, LIMIT_S, STEP = 20 * 1024 * 1024, 45 * 60, 5
DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID", "SEMIO_SCRIPT_STALL_MS")
PATTERNS = {
    "refreshing": re.compile(r"graph: refreshing (\d+) projects"),
    "inferring": re.compile(r"Inferring projects: \d+ of (\d+) projects"),
    "published": re.compile(r"graph: generation (\d+) published, (\d+) projects refreshed in (\d+) s"),
    "actions": re.compile(r"Build completed successfully, (\d+) total actions"),
    "processes": re.compile(r"^INFO: (\d+) processes?: (.*?)\.?$"),
}


def tree_kb(root):
    rows = {}
    for line in subprocess.run(["ps", "-A", "-o", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.splitlines():
        cells = line.split()
        if len(cells) == 3 and all(c.isdigit() for c in cells):
            rows[int(cells[0])] = (int(cells[1]), int(cells[2]))
    kids = {}
    for pid, (ppid, _) in rows.items():
        kids.setdefault(ppid, []).append(pid)
    total, stack, count = 0, [root], 0
    while stack:
        pid = stack.pop()
        if pid in rows:
            total, count = total + rows[pid][1], count + 1
        stack.extend(kids.get(pid, []))
    return total, count


def footprints():
    rows, out = {}, subprocess.run(["top", "-l", "1", "-stats", "pid,ppid,mem"], capture_output=True, text=True).stdout.splitlines()
    for line in out:
        cells = line.split()
        if len(cells) == 3 and cells[0].isdigit() and cells[1].isdigit():
            value = cells[2].rstrip("+-")
            if value and value[-1] in UNITS:
                try: rows[int(cells[0])] = (int(cells[1]), float(value[:-1]) * UNITS[value[-1]])
                except ValueError: pass
    return rows


def tree_pids(root, parents):
    kids = {}
    for pid, (ppid, _) in parents.items(): kids.setdefault(ppid, []).append(pid)
    seen, stack = [], [root]
    while stack:
        pid = stack.pop(); seen.append(pid); stack.extend(kids.get(pid, []))
    return seen


def command_of(pid):
    text = subprocess.run(["ps", "-o", "command=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    for marker in ("--label", "build:graph-plan", "repository:command", "bazel", "cargo", "rustc"):
        if marker in text: return marker + " | " + text[-120:]
    return text[:160]


def queued(log):
    if not os.path.exists(log):
        return False
    with open(log, "rb") as stream:
        stream.seek(0, 2)
        stream.seek(max(0, stream.tell() - 600))
        tail = stream.read().decode("utf8", "replace").strip().splitlines()[-1:] or [""]
    return "[capacity] waiting" in tail[0] or "a peer builds the graph" in tail[0]


def main(label, target):
    os.makedirs(OUT, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith(DROP)}
    env.setdefault("SEMIO_LANGUAGE", "en")
    log = os.path.join(OUT, label + ".log")
    started, t0 = time.strftime("%H:%M:%S"), time.monotonic()
    handle = open(log, "wb", buffering=0)
    child = subprocess.Popen(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run", target], cwd=ROOT, env=env, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
    peak, peak_n, killed, parked, tick = 0, 0, None, 0.0, time.monotonic()
    peak_fp, procs = 0, {}
    while child.poll() is None:
        now = time.monotonic()
        parked += (now - tick) if queued(log) else 0.0
        tick = now
        kb, n = tree_kb(child.pid)
        peak, peak_n = max(peak, kb), max(peak_n, n)
        fps = footprints()
        pids = tree_pids(child.pid, fps)
        peak_fp = max(peak_fp, sum(fps[p][1] for p in pids if p in fps))
        rss = {int(c[0]): int(c[1]) for c in (l.split() for l in subprocess.run(["ps", "-A", "-o", "pid=,rss="], capture_output=True, text=True).stdout.splitlines()) if len(c) == 2 and c[0].isdigit() and c[1].isdigit()}
        for p in pids:
            if p in fps:
                row = procs.setdefault(p, {"pid": p, "rss_mb": 0, "fp_mb": 0, "cmd": None})
                row["rss_mb"], row["fp_mb"] = max(row["rss_mb"], rss.get(p, 0) // 1024), max(row["fp_mb"], int(fps[p][1] // 1024))
                if row["cmd"] is None and row["fp_mb"] > 300: row["cmd"] = command_of(p)
        if kb > LIMIT_KB or time.monotonic() - t0 - parked > LIMIT_S:
            killed = "rss" if kb > LIMIT_KB else "time"
            os.killpg(child.pid, signal.SIGTERM)
            time.sleep(10)
            if child.poll() is None:
                os.killpg(child.pid, signal.SIGKILL)
            break
        time.sleep(STEP)
    wall = round(time.monotonic() - t0, 1)
    found = {}
    with open(log, encoding="utf8", errors="replace") as stream:
        for line in stream:
            for key, pattern in PATTERNS.items():
                match = pattern.search(line)
                if match:
                    found[key] = match.groups()
    record = {"label": label, "target": target, "started": started, "wall_s": wall, "peak_tree_mb": peak // 1024, "peak_tree_footprint_mb": int(peak_fp // 1024), "largest": sorted(procs.values(), key=lambda r: -r["fp_mb"])[:5], "peak_processes": peak_n, "exit": child.returncode, "killed": killed, "queued_s": round(parked, 1), "graph": found}
    json.dump(record, open(os.path.join(OUT, label + ".json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(json.dumps(record, ensure_ascii=False))


main(sys.argv[1], sys.argv[2])
