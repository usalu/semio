"""Writes the QA11 render job list (every pose + 3 clip instants, every authored scale) and the asset file list."""
import glob
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🚜️vehicles")
jobs = []
files = []
for f in sorted(glob.glob(os.path.join(CATALOG, "*", "🔣️.json"))):
    a = json.load(open(f, encoding="utf-8"))["asset"]
    files.append(f)
    out = os.path.join(T, "🗑️generated", "QA11", a["id"])
    for scale, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        base = ["--file", f, "--scale", scale, "--views", "--dpi", "150", "--out", out]
        for p in a["poses"]:
            jobs.append({"argv": base + ["--pose", p["id"]]})
        for c in a.get("clips", []):
            d = c.get("durationS", c.get("duration"))
            for fr in (0.15, 0.5, 0.9):
                jobs.append({"argv": base + ["--clip", c["id"], "--time", f"{d * fr:.3f}"]})
json.dump(jobs, open(os.path.join(HERE, "jobs.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
json.dump(files, open(os.path.join(HERE, "files.json"), "w", encoding="utf-8"), ensure_ascii=False)
print(len(jobs), "jobs", len(files), "files")
