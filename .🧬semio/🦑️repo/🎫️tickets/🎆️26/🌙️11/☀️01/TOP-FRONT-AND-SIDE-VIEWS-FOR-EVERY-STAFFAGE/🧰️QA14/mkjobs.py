import json, os, sys
sys.stdout.reconfigure(encoding="utf8")
here = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(here)
out = (T + "/🗑️generated/C2/qa14").replace("\\", "/")
R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"
ids = sys.argv[2:] or "steel-column sliding-door tilt-window tied-canopy wheelchair-ramp stair-flight spiral-stair".split()
mode = sys.argv[1]
jobs = []
for a in ids:
    d = json.load(open(R + a + "/🔣️.json", encoding="utf8"))["asset"]
    scales = [int(k) for k, v in d["representations"].items() if v.get("status") == "authored"]
    for sc in sorted(scales):
        base = ["--asset", a, "--scale", str(sc), "--views", "--no-png", "--out", out + "/" + a]
        for p in d["poses"]:
            jobs.append(["--pose", p["id"]] + base)
        for c in d["clips"]:
            dur = c["durationS"]
            ts = {round(dur * f, 2) for f in (0.25, 0.5, 0.75)}
            keys = [k["t"] for k in c.get("keys", [])]
            if len(keys) <= 5: ts |= set(keys)
            if a == "spiral-stair" and c["id"] == "climb": ts |= {0, 1, 14}
            for t in sorted(ts):
                jobs.append(["--clip", c["id"], "--time", str(t)] + base)
print(len(jobs))
json.dump(jobs, open(here + "/jobs-" + mode + ".json", "w", encoding="utf8"), ensure_ascii=False)
