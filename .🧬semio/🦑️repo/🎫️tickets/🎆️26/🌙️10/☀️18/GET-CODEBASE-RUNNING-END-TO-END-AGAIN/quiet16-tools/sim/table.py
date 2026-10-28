"""📋️ quiet16 (derived from agents16's table): runs the simulation for every scenario x host against one checkout and renders the outcome table (markdown) and, with --law, judges it like harness3's criteria: no run queued more than twice its own time above 120 s (not in `busy2`/`busy3`, whose budget carries one edit at a time, so queueing is what they demand), no wait row of 60 s with nothing of ours running, the third agent's first hot no-op done within 15 s on the gate's host, every wait row naming what it stands behind (only with --named); with --quiet: every hot no-op (a run named `*-noop`) waits at most one door poll (1 s) for memory (door and growth) on every host, the declared 16 GB laptop included (a peer's freshly grown edit counts its whole reservation while it ramps); with --lastrun also: every no-op is forecast quiet (the last run of its class executed nothing or ended within the follow-up window); with --busy (audit9 §hold): in `busy2`/`busy3` (two or three quiet-forecast runs that turn busy at once on a 16 GB laptop with 9 GB foreign, `h16-busy`, and on `h16`) and on `h16-tight` (11 GB foreign, where growths are admitted partly) the host's free memory never falls more than one job's footprint below the floor: what our runs really hold stays within the budget plus one job.
Usage: python3 -I table.py <root> <label> [--law] [--named] [--quiet [--lastrun]] [--busy] [--scenarios a,b] [--hosts x,y]   (JSON outputs go to $QUIET16_OUT, default C/🗑️generated/quiet16)"""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, "..", ".."))
OUT = os.environ.get("QUIET16_OUT") or os.path.join(C, "🗑️generated", "quiet16")
SCENARIOS = ["hot", "hot-firstedit", "hot-late", "hot-third", "late-third", "mixed", "mixed-long", "cold", "cold-followup", "busy2", "busy3"]
HOSTS = ["h32d-fleet", "h32d-quiet", "h16"]
SCENARIO_HOSTS = {"busy2": ["h16-busy", "h16-tight", "h16"], "busy3": ["h16-busy", "h16-tight", "h16"]}
DATA = json.load(open(os.path.join(HERE, "..", "data", "scenarios.json"), encoding="utf8"))


def one_job(scenario):
    """🧮️ The footprint of one job of the widest executing segment the scenario's agents run: the growth of the segment above its start level at the recorded 8 jobs, divided by 8."""
    worst = 0
    for agent in DATA["scenarios"][scenario]["agents"]:
        for step in agent["steps"]:
            segments = DATA["runs"][step]["segments"]
            for index, segment in enumerate(segments):
                if segment.get("executes"): worst = max(worst, (segment["fp"] - (segments[index - 1]["fp"] if index else 120)) / 8)
    return worst


def option(name, default):
    return sys.argv[sys.argv.index(name) + 1].split(",") if name in sys.argv else default


def run(root, scenario, host, label):
    out = os.path.join(OUT, "sim-%s-%s-%s.json" % (label, scenario, host))
    text = subprocess.run(["bun", os.path.join(HERE, "sim.ts"), "--root", root, "--scenario", scenario, "--host", host, "--json", out], cwd=root, capture_output=True, text=True, check=True).stdout
    return json.loads(text.strip().splitlines()[-1])


def main():
    root, label = os.path.abspath(sys.argv[1]), sys.argv[2]
    rows, failures = [], []
    print("| scenario | host | makespan s | runs: wall s (alone) +queue s, jobs | wait rows (naming a blocker) | reasons | starved | peak reserved/committed MB vs real footprint MB | min free | max tree RSS MB | max real above committed MB |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for scenario in option("--scenarios", SCENARIOS):
        for host in option("--hosts", SCENARIO_HOSTS.get(scenario, HOSTS)):
            d = run(root, scenario, host, label)
            runs = "; ".join("%s %s %.0f (%.0f) +%.0f, %dj" % (r["agent"], r["run"], r["wall"], r["alone"], r["door"] + r["upgrade"], r["jobs"]) + (" +%.0f graph" % r["custody"] if r["custody"] else "") for r in d["runs"])
            starved = d["queueOver2x"] + (["%d rows >= 60 s alone" % d["starvedRows"]] if d["starvedRows"] else [])
            print("| %s | %s | %.0f | %s | %d (%d) | %s | %s | %d vs %d | %.1f %% | %d | %d |" % (scenario, host, d["makespan"], runs, d["waitRows"], d["waitRowsNamingBlocker"], ", ".join("%s %d" % kv for kv in d["waitReasons"].items()) or "-", ", ".join(starved) or "none", d["peakCommitted"]["committed"], d["peakReal"]["real"], d["minFree"]["freePercent"], d["maxRss"]["rss"], d["maxExcess"]["excess"]))
            rows.append(d)
            if starved and scenario not in SCENARIO_HOSTS:
                failures.append("%s/%s starved: %s" % (scenario, host, starved))
            if scenario == "hot" and host == "h32d-fleet":
                first = next(r for r in d["runs"] if r["agent"] == "C")
                if first["wall"] > 15:
                    failures.append("hot/h32d-fleet: C's first no-op took %.0f s (alone %.0f s)" % (first["wall"], first["alone"]))
            if "--busy" in sys.argv and scenario in SCENARIO_HOSTS and d["minFree"]["free"] < d["floorMegabytes"] - one_job(scenario):
                failures.append("%s/%s: the host's free memory fell to %d MB at %.0f s, more than one job (%d MB) below the %d MB floor (our runs held %d MB beyond what the gate committed)" % (scenario, host, d["minFree"]["free"], d["minFree"]["t"], one_job(scenario), d["floorMegabytes"], d["maxExcess"]["excess"]))
            if "--quiet" in sys.argv:
                for r in d["runs"]:
                    if r["run"].endswith("-noop") and not r["quiet"] and "--lastrun" in sys.argv:
                        failures.append("%s/%s: %s's no-op %s was not forecast quiet although the last run of its class executed nothing" % (scenario, host, r["agent"], r["run"]))
                    if r["run"].endswith("-noop") and r["door"] + r["upgrade"] > 1.0:
                        failures.append("%s/%s: %s's no-op %s waited %.1f s for memory (door %.1f, growth %.1f)" % (scenario, host, r["agent"], r["run"], r["door"] + r["upgrade"], r["door"], r["upgrade"]))
            if "--named" in sys.argv and d["waitRows"] != d["waitRowsNamingBlocker"]:
                failures.append("%s/%s: %d of %d wait rows name no blocker" % (scenario, host, d["waitRows"] - d["waitRowsNamingBlocker"], d["waitRows"]))
    json.dump(rows, open(os.path.join(OUT, "table-%s.json" % label), "w"), indent=1)
    if "--law" in sys.argv:
        for line in failures:
            print("LAW FAIL", line)
        print("LAW %s (%s)" % ("PASS" if not failures else "FAIL", label))
        sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
