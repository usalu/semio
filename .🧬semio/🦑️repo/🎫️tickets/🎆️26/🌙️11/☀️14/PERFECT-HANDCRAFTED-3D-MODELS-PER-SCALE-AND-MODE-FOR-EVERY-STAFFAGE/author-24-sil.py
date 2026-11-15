"""Silhouette experiment: python3 author-24-sil.py <slug> <scale> '<python on body list b>'  (restores the source)"""
import sys, json
sys.path.insert(0, ".")
from author_24_lib import *
slug, scale, expr = sys.argv[1], sys.argv[2], sys.argv[3]
path = f"{T}/volumes/humans/{slug}.json"
orig = open(path).read()
try:
    v = json.loads(orig)
    b = v["volumes"][scale]["body"]
    exec(expr)
    v["volumes"][scale]["body"] = b
    json.dump(v, open(path, "w"), ensure_ascii=False, separators=(",", ":"))
    for m in measurements(slug, int(scale), "volumes-silhouette,volumes-bounds"):
        if m["name"].startswith(("iou", "p95M", "bboxDev")): print(m["name"], round(m["value"], 4), m["limit"])
finally:
    open(path, "w").write(orig)
