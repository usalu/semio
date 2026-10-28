#!/usr/bin/env python3
"""⏱️ Startup benchmark of the implementation modes (run INSIDE a mirror whose sealed snapshot is fresh: mm run <n> -- python3 -I bench.py [runs [command words]]).
Interleaves live and sealed runs of each command (so load drift hits both alike), prints wall seconds, the largest single-process RSS (wait4 rusage) and the median.
Streams one line per run; no output kept in memory beyond the last line."""
import os, re, statistics, subprocess, sys, time
RUNS = int(sys.argv[1]) if len(sys.argv) > 1 else 5
COMMANDS = {"repo --help": ["bun", "repo", "--help"], "repo leases status": ["bun", "repo", "leases", "status"]}
if len(sys.argv) > 2:
    COMMANDS = {" ".join(sys.argv[2:]): ["bun", "repo"] + sys.argv[2:]}
MODES = ("live", "sealed")
rows = {(c, m): [] for c in COMMANDS for m in MODES}
for n in range(RUNS):
    for name, argv in COMMANDS.items():
        for mode in MODES if n % 2 == 0 else MODES[::-1]:
            env = dict(os.environ, SEMIO_REPO_IMPLEMENTATION=mode, SEMIO_LANGUAGE="en")
            started = time.perf_counter()
            done = subprocess.run(["/usr/bin/time", "-l"] + argv, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
            wall = time.perf_counter() - started
            match = re.search(r"(\d+)\s+maximum resident set size", done.stderr)
            rss = int(match.group(1)) / 1048576 if match else float("nan")
            promoted = "Promot" in done.stderr
            rows[(name, mode)].append((wall, rss))
            print("run %d  %-20s %-6s exit %d  wall %6.2f s  peak process RSS %6.0f MB%s" % (n + 1, name, mode, done.returncode, wall, rss, "  PROMOTED" if promoted else ""), flush=True)
print()
for (name, mode), values in rows.items():
    print("%-20s %-6s median wall %6.2f s (min %5.2f, max %5.2f)  median peak process RSS %5.0f MB" % (name, mode, statistics.median(v[0] for v in values), min(v[0] for v in values), max(v[0] for v in values), statistics.median(v[1] for v in values)))
