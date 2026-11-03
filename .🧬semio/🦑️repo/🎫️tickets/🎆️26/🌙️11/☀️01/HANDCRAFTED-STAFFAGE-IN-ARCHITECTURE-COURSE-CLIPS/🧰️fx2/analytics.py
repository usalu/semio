import json, pathlib, collections
S = pathlib.Path(r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog")
m = json.loads((S / "🔣️.json").read_text(encoding="utf-8"))
bad = []
pat = collections.Counter()
for e in m["entries"]:
    a = json.loads((S / next(p.name for p in S.iterdir() if p.is_dir() and (p / e["id"]).is_dir()) / e["id"] / "🔣️.json").read_text(encoding="utf-8"))["asset"]
    th = len(a["thermal"]["regions"]) > 0
    tr = len(a["thermal"]["emitters"]) > 0
    st = "structural" in e["analytics"]
    pat[tuple(sorted(e["analytics"]))] += 1
    if ("thermal" in e["analytics"]) != th or ("transport" in e["analytics"]) != tr:
        bad.append((e["id"], e["analytics"], th, tr))
print(len(m["entries"]), "entries")
print(pat)
for b in bad:
    print("MISMATCH", b)
