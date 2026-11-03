import json, sys, os
here = os.path.dirname(os.path.abspath(__file__))
out = os.path.join(os.path.dirname(here), "🗑️generated", "C2", "prev").replace("\\", "/")
only = sys.argv[1:]
plan = {
    "steel-column": ([50, 100, 200, 500], ["eccentric-load"], []),
    "sliding-door": ([50, 100, 200, 500], ["closed", "open"], [("slide-open-close", 3.0)]),
    "tilt-window": ([50, 100, 200], ["closed", "fully-tilted"], [("tilt-open-close", 1.5), ("tilt-open-close", 2.5)]),
    "tied-canopy": ([50, 100, 200, 500], ["snow-load"], []),
    "wheelchair-ramp": ([50, 100, 200, 500], ["self-weight"], [("wheelchair-ascent", 5.0)]),
    "stair-flight": ([50, 100, 200, 500], ["crowd-live-load"], [("moving-point-load", 3.0)]),
    "spiral-stair": ([50, 100, 200, 500], ["self-weight", "walker-on-tread-8"], [("climb", 7.0)]),
}
jobs = []
for a, (scs, poses, clips) in plan.items():
    if only and a not in only:
        continue
    for sc in scs:
        for p in poses:
            if sc != 50 and p != poses[0]:
                continue
            jobs.append(["--asset", a, "--scale", str(sc), "--pose", p, "--views", "--out", out + "/" + a])
        if sc == 50:
            for c, t in clips:
                jobs.append(["--asset", a, "--scale", str(sc), "--clip", c, "--time", str(t), "--views", "--out", out + "/" + a])
json.dump(jobs, open(os.path.join(here, "jobs.json"), "w", encoding="utf8"), ensure_ascii=False)
print(len(jobs))
