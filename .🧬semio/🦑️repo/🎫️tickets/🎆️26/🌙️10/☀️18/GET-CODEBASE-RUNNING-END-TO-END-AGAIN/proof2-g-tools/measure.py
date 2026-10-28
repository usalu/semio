"""📏 One measured official run: `measure.py <outdir> <label> <cmd...>` writes <outdir>/<label>.log and <label>.json.

The command runs in its own session under `nice -n 10`, its process-tree RSS is sampled every 3 s (streaming, no logs in memory), and
only its own process group is stopped above 20 GB or after the wall limit (PROOF_LIMIT_S, default 2700). The log is scanned line by line for
graph rows, Bazel action counts, Nx cache rows and the capacity notices."""
import json, os, re, resource, signal, subprocess, sys, time

DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID", "SEMIO_SCRIPT_STALL_MS")
PATTERNS = {
    "generation": re.compile(r"graph: (serving generation \d+.*|generation \d+ published.*|refreshing \d+ projects.*)"),
    "inferring": re.compile(r"Inferring projects: \d+ of (\d+) projects"),
    "actions": re.compile(r"Build completed successfully, (\d+) total actions"),
    "processes": re.compile(r"^INFO: (\d+) processes?: (.*?)\.?$"),
    "nx": re.compile(r"(NX .*(Successfully ran|Cache|cache).*|\d+ (tasks?|cache hits?).*)"),
    "capacity": re.compile(r"^\[capacity\] (.*)"),
    "waiting": re.compile(r"(waits? for|Waiting for|queued).*"),
}


CENSUS = re.compile(os.environ["PROOF_CENSUS"]) if os.environ.get("PROOF_CENSUS") else None
CENSUS_MAX = {"count": 0, "samples": []}


def tree(root):
    rows = {}
    census = []
    for line in subprocess.run(["ps", "-A", "-o", "pid=,ppid=,rss=,command="], capture_output=True, text=True).stdout.splitlines():
        cells = line.split(None, 3)
        if len(cells) >= 3 and all(cell.isdigit() for cell in cells[:3]):
            rows[int(cells[0])] = (int(cells[1]), int(cells[2]))
            if CENSUS and len(cells) == 4 and CENSUS.search(cells[3]):
                census.append((int(cells[0]), int(cells[1]), cells[3][:160]))
    TREE_CENSUS[:] = census
    kids = {}
    for pid, (ppid, _) in rows.items():
        kids.setdefault(ppid, []).append(pid)
    total, count, stack, members = 0, 0, [root], set()
    while stack:
        pid = stack.pop()
        if pid in rows:
            total, count = total + rows[pid][1], count + 1
            members.add(pid)
        stack.extend(kids.get(pid, []))
    mine = [row for row in TREE_CENSUS if row[0] in members]
    if len(mine) > CENSUS_MAX["count"]:
        CENSUS_MAX["count"], CENSUS_MAX["samples"] = len(mine), mine[:12]
    return total / 1024, count


TREE_CENSUS: list = []


def main() -> None:
    outdir, label, argv = sys.argv[1], sys.argv[2], sys.argv[3:]
    os.makedirs(outdir, exist_ok=True)
    env = {key: value for key, value in os.environ.items() if not key.startswith(DROP)}
    env.setdefault("SEMIO_LANGUAGE", "en")
    log = os.path.join(outdir, label + ".log")
    limit = float(os.environ.get("PROOF_LIMIT_S", "2700"))
    started, begin = time.strftime("%H:%M:%S"), time.monotonic()
    with open(log, "wb", buffering=0) as handle:
        child = subprocess.Popen(["nice", "-n", "10", *argv], env=env, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
        peak, procs, killed, series = 0.0, 0, None, []
        while child.poll() is None:
            mb, count = tree(child.pid)
            peak, procs = max(peak, mb), max(procs, count)
            series.append([round(time.monotonic() - begin), count, round(mb)])
            if mb > 20480 or time.monotonic() - begin > limit:
                killed = "rss" if mb > 20480 else "time"
                os.killpg(child.pid, signal.SIGTERM)
                time.sleep(10)
                if child.poll() is None:
                    os.killpg(child.pid, signal.SIGKILL)
                break
            time.sleep(3)
    wall = round(time.monotonic() - begin, 1)
    found = {key: [] for key in PATTERNS}
    with open(log, encoding="utf8", errors="replace") as stream:
        for line in stream:
            for key, pattern in PATTERNS.items():
                hit = pattern.search(line)
                if hit and (not found[key] or found[key][-1] != hit.group(0)) and len(found[key]) < 12:
                    found[key].append(hit.group(0)[:200])
    row = {"label": label, "argv": argv, "started": started, "wall": wall, "exit": child.returncode, "killed": killed, "peakMB": round(peak), "maxProcs": procs, "found": found, "series": series[-120:], "censusMax": CENSUS_MAX, "ownRssMB": round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1048576)}
    with open(os.path.join(outdir, label + ".json"), "w") as stream:
        json.dump(row, stream)
    print(json.dumps({key: row[key] for key in ("label", "started", "wall", "exit", "killed", "peakMB", "maxProcs")}))


main()
