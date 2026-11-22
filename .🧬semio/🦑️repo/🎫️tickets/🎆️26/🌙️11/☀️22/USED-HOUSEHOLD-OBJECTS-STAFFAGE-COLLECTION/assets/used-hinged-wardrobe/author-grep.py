"""🔎 Shows bones, poses and shapes of a catalog asset mentioning a word: python3 author-grep.py <cat>/<slug> <word>."""
import json, sys, glob
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
cat, slug = sys.argv[1].split("/")
a = json.load(open(glob.glob(f"{S}/*{cat}/{slug}/🔣️.json")[0]))["asset"]
w = sys.argv[2]
for b in a["rig"]["bones"]:
    if w in b["id"]: print("BONE", json.dumps(b, ensure_ascii=False)[:600])
for p in a["poses"]:
    s = json.dumps(p["bones"]); 
    if w in s: print("POSE", p["id"], s[:400])
r = a["representations"]["50"]
for v in ("elevation", "front", "plan"):
    for sh in r["geometry"][v]:
        if w in sh["bone"]: print("SHAPE", v, json.dumps(sh, ensure_ascii=False)[:500])
for k in ("body",):
    for so in a["volumes"]["50"][k]:
        if w in so["bone"]: print("SOLID", json.dumps(so, ensure_ascii=False)[:500])
