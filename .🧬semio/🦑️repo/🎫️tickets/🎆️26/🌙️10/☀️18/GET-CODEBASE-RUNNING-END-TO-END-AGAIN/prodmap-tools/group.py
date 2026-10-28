"""🗺️ One measured official run-many group: `group.py <label> <projects,comma> <targets,comma>`; same sampling and verdict record as `gate-g-tools/product.py`, writes `🗑️generated/prodmap-run/<label>.{log,json}`.

Samples the process-tree RSS every 5 s, kills only its own process group above 20 GB or after GROUP_LIMIT_MIN (default 90) minutes of running time (time parked in the capacity or graph queue does not count) and records wall, graph rows and exit."""
import json, os, re, signal, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
OUT = os.path.join(os.path.dirname(HERE), "🗑️generated", "prodmap-run")
LIMIT_KB, LIMIT_S, STEP = 20 * 1024 * 1024, int(os.environ.get("GROUP_LIMIT_MIN", "90")) * 60, 5
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


def queued(log):
    if not os.path.exists(log):
        return False
    with open(log, "rb") as stream:
        stream.seek(0, 2)
        stream.seek(max(0, stream.tell() - 600))
        tail = stream.read().decode("utf8", "replace").strip().splitlines()[-1:] or [""]
    return "[capacity] waiting" in tail[0] or "a peer builds the graph" in tail[0]


def main(label, projects, targets):
    os.makedirs(OUT, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith(DROP)}
    env.setdefault("SEMIO_LANGUAGE", "en")
    log = os.path.join(OUT, label + ".log")
    started, t0 = time.strftime("%H:%M:%S"), time.monotonic()
    handle = open(log, "wb", buffering=0)
    child = subprocess.Popen(["nice", "-n", "10", "bun", "repo", "--tool=bazel", "run-many", "--projects=" + projects, "--targets=" + targets, "--parallel=1"], cwd=ROOT, env=env, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
    peak, peak_n, killed, parked, tick = 0, 0, None, 0.0, time.monotonic()
    while child.poll() is None:
        now = time.monotonic()
        parked += (now - tick) if queued(log) else 0.0
        tick = now
        kb, n = tree_kb(child.pid)
        peak, peak_n = max(peak, kb), max(peak_n, n)
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
    record = {"label": label, "target": projects + " :: " + targets, "started": started, "wall_s": wall, "peak_tree_mb": peak // 1024, "peak_processes": peak_n, "exit": child.returncode, "killed": killed, "queued_s": round(parked, 1), "graph": found}
    json.dump(record, open(os.path.join(OUT, label + ".json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(json.dumps(record, ensure_ascii=False))


main(sys.argv[1], sys.argv[2], sys.argv[3])
