"""📋️ agents16: runs the simulation for every scenario x host against one checkout and renders the outcome table (markdown) and, with --law, judges it like harness3's criteria: no run queued more than twice its own time above 120 s, no wait row of 60 s with nothing of ours running, the third agent's first hot no-op done within 15 s on the gate's host, every wait row naming what it stands behind (only with --named).
Usage: python3 -I table.py <root> <label> [--law] [--named] [--scenarios a,b] [--hosts x,y]"""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, "..", ".."))
SCENARIOS = ["hot", "hot-firstedit", "hot-late", "mixed", "mixed-long", "cold", "cold-followup"]
HOSTS = ["h32d-fleet", "h32d-quiet", "h16"]


def option(name, default):
    return sys.argv[sys.argv.index(name) + 1].split(",") if name in sys.argv else default


def run(root, scenario, host, label):
    out = os.path.join(C, "🗑️generated", "agents16", "sim-%s-%s-%s.json" % (label, scenario, host))
    text = subprocess.run(["bun", os.path.join(HERE, "sim.ts"), "--root", root, "--scenario", scenario, "--host", host, "--json", out], cwd=root, capture_output=True, text=True, check=True).stdout
    return json.loads(text.strip().splitlines()[-1])


def main():
    root, label = os.path.abspath(sys.argv[1]), sys.argv[2]
    rows, failures = [], []
    print("| scenario | host | makespan s | runs: wall s (alone) +queue s, jobs | wait rows (naming a blocker) | reasons | starved | peak reserved/committed MB vs real footprint MB | min free | max tree RSS MB |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for scenario in option("--scenarios", SCENARIOS):
        for host in option("--hosts", HOSTS):
            d = run(root, scenario, host, label)
            runs = "; ".join("%s %s %.0f (%.0f) +%.0f, %dj" % (r["agent"], r["run"], r["wall"], r["alone"], r["door"] + r["upgrade"], r["jobs"]) + (" +%.0f graph" % r["custody"] if r["custody"] else "") for r in d["runs"])
            starved = d["queueOver2x"] + (["%d rows >= 60 s alone" % d["starvedRows"]] if d["starvedRows"] else [])
            print("| %s | %s | %.0f | %s | %d (%d) | %s | %s | %d vs %d | %.1f %% | %d |" % (scenario, host, d["makespan"], runs, d["waitRows"], d["waitRowsNamingBlocker"], ", ".join("%s %d" % kv for kv in d["waitReasons"].items()) or "-", ", ".join(starved) or "none", d["peakCommitted"]["committed"], d["peakReal"]["real"], d["minFree"]["freePercent"], d["maxRss"]["rss"]))
            rows.append(d)
            if starved:
                failures.append("%s/%s starved: %s" % (scenario, host, starved))
            if scenario == "hot" and host == "h32d-fleet":
                first = next(r for r in d["runs"] if r["agent"] == "C")
                if first["wall"] > 15:
                    failures.append("hot/h32d-fleet: C's first no-op took %.0f s (alone %.0f s)" % (first["wall"], first["alone"]))
            if "--named" in sys.argv and d["waitRows"] != d["waitRowsNamingBlocker"]:
                failures.append("%s/%s: %d of %d wait rows name no blocker" % (scenario, host, d["waitRows"] - d["waitRowsNamingBlocker"], d["waitRows"]))
    json.dump(rows, open(os.path.join(C, "🗑️generated", "agents16", "table-%s.json" % label), "w"), indent=1)
    if "--law" in sys.argv:
        for line in failures:
            print("LAW FAIL", line)
        print("LAW %s (%s)" % ("PASS" if not failures else "FAIL", label))
        sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
