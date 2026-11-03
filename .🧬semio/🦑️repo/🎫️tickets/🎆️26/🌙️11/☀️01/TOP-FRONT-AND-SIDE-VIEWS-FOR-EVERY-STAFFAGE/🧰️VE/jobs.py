"""Writes a render job list for render.ts.

usage: python jobs.py <asset> <scales,..> <queries,..> <out-jobs.json>
query: pose:<id> or clip:<id>@<t1>/<t2>/..   (all views sheets)
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🚜️vehicles")

asset, scales, queries, out = sys.argv[1], sys.argv[2].split(","), sys.argv[3].split(","), sys.argv[4]
jobs = []
for scale in scales:
    for q in queries:
        kind, rest = q.split(":", 1)
        base = ["--file", os.path.join(CATALOG, asset, "🔣️.json"), "--scale", scale, "--views", "--out", os.path.join(T, "🗑️generated", "VE", asset)]
        if kind == "pose":
            jobs.append({"argv": base + ["--pose", rest]})
        else:
            clip, times = rest.split("@")
            for t in times.split("/"):
                jobs.append({"argv": base + ["--clip", clip, "--time", t]})
with open(out, "w", encoding="utf-8") as f:
    json.dump(jobs, f, ensure_ascii=False, indent=1)
print(len(jobs), "jobs")
