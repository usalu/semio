"""usage: python sel.py <out.json> <substring> ... : keeps the jobs whose argv contains one of the (asset|pose|clip) substrings, e.g. city-bicycle/pose=parked-on-stand"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
jobs = json.load(open(os.path.join(HERE, "jobs.json"), encoding="utf-8"))
out, wants = sys.argv[1], [w.split("/") for w in sys.argv[2:]]
sel = []
for j in jobs:
    a = j["argv"]
    asset = a[1].split(os.sep)[-2]
    pc = a[a.index("--pose") + 1] if "--pose" in a else a[a.index("--clip") + 1]
    for w_asset, w_pc in wants:
        if asset == w_asset and (w_pc == "*" or pc == w_pc):
            sel.append(j)
            break
json.dump(sel, open(os.path.join(HERE, out), "w", encoding="utf-8"), ensure_ascii=False)
print(len(sel), "jobs")
