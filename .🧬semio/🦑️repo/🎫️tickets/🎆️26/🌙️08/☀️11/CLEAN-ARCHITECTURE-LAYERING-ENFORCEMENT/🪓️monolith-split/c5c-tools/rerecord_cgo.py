"""🧾 C5C re-record of the child-group-owner `remaining` entry of the split atc root: the entry becomes the root plus its fragments in include order; the concatenated function list must equal the old list name for name, with every body equal modulo the `../` rebase. usage: rerecord_cgo.py [--apply]"""
import json, os, re, sys, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
G = os.path.dirname(HERE) + "/🗑️generated/"
R = "/Users/ueli/Documents/semio/"
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
path = R + PL + "🧪️tests/📦️units/📋️child-group-owner.json"
d = json.load(open(path, encoding="utf-8"))
new = json.load(open(G + "c5c-fnrows-atc-all.json", encoding="utf-8"))
old = list(json.load(open(G + "c5c-fnrows-atc-old.json", encoding="utf-8")).values())[0]["fns"]
files = [l.strip() for l in open(HERE + "/atc-paths.txt", encoding="utf-8") if l.strip()]
flat = [(f, r) for f in files for r in new[f]["fns"]]
assert len(flat) == len(old), (len(flat), len(old))
unrebase = lambda t: re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', t)
ch = 0
for (f, r), o in zip(flat, old):
    assert r["name"] == o["name"], (r["name"], o["name"])
    assert unrebase(r["text"]) == o["text"], r["name"]
    ch += r["sha256"] != o["sha256"]
entries = []
for f in files:
    text = open(f, "rb").read()
    entries.append({"path": os.path.relpath(f, R), "sha256": hashlib.sha256(text).hexdigest(), "functions": [{"name": r["name"], "sha256": r["sha256"]} for r in new[f]["fns"]]})
atc_rel = PL + "🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs"
idx = [i for i, r in enumerate(d["remaining"]) if r["path"] == atc_rel]
assert len(idx) == 1
assert [x["sha256"] for x in d["remaining"][idx[0]]["functions"]] == [o["sha256"] for o in old], "old record differs from the old file"
print(f"functions {len(flat)} (re-recorded sha {ch}), entries {len(entries)}")
if "--apply" in sys.argv:
    d["remaining"][idx[0]:idx[0] + 1] = entries
    open(path, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print("written")
