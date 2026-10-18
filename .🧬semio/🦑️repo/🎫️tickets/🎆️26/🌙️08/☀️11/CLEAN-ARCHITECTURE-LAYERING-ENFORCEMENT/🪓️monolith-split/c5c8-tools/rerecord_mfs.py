"""🧾 C5C8 re-record of the plugin units `surface-fixture` owner records after the include-fragment cut of `🧬️mutation-fixtures-surface`.
Proof per function: the fragment function text equals the recorded original text or equals it after reversing the `../` rebase of an include literal. Keeps files[0] (root, now without functions except the metadata law) and appends the 7 fragments in include order.
usage: rerecord_mfs.py [--apply]   (rows: /Users/ueli/.cache/c5c8/rows-mfs.json from fnrows-text.ts over the snapshot, the root, the fragments and the metadata target)"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio/"
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
U = R + PL + "🧪️tests/📦️units/"
MFS = PL + "🧪️tests/🧬️mutation-fixtures-surface/"
apply = "--apply" in sys.argv
unrebase = lambda t: re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', t)
rows = json.load(open("/Users/ueli/.cache/c5c8/rows-mfs.json", encoding="utf-8"))
orig = rows[HERE + "/orig/mfs.rs"]
root_rel = MFS + "🦀️.rs"
root = rows[R + root_rel]
root_text = open(R + root_rel, encoding="utf-8").read()
incl = re.findall(r'^include!\("([^"]+)"\);$', root_text, re.M)
assert len(incl) == 7, incl
frag_rel = [MFS + i for i in incl]
path = U + "📋️surface-fixture.json"
d = json.load(open(path, encoding="utf-8"))
meta_rel = d["metadataTarget"]
meta = rows[R + meta_rel]
f0 = d["files"][0]
assert f0["path"] == root_rel
old_total = sum(f["assertions"] for f in d["files"])
assert f0["assertions"] == orig["assertions"] + meta["assertions"], (f0["assertions"], orig["assertions"], meta["assertions"])
mine = [fn for fn in f0["functions"] if fn["target"] == root_rel]
others = [fn for fn in f0["functions"] if fn["target"] != root_rel]
assert len(mine) == len(orig["fns"]) and len(others) == 1, (len(mine), len(orig["fns"]), len(others))
for rec, o in zip(mine, orig["fns"]):
    assert rec["name"] == o["name"] and rec["sha256"] == o["sha256"] and rec["assertions"] == o["assertions"], rec["name"]
new_fns = []
for rel in frag_rel:
    r = rows[R + rel]
    assert not r["hasError"], rel
    for fn in r["fns"]:
        new_fns.append((rel, fn))
assert [fn["name"] for _, fn in new_fns] == [o["name"] for o in orig["fns"]], "function order differs"
same = changed = 0
per = {rel: [] for rel in frag_rel}
for (rel, fn), o, rec in zip(new_fns, orig["fns"], mine):
    if fn["text"] == o["text"]:
        same += 1
    else:
        assert unrebase(fn["text"]) == o["text"], ("text differs beyond the rebase", fn["name"])
        changed += 1
    assert fn["assertions"] == rec["assertions"], fn["name"]
    per[rel].append(dict(name=rec["name"], sha256=fn["sha256"], assertions=rec["assertions"], law=rec["law"], target=rel))
assert not root["hasError"] and not root["fns"]
frag_assert = sum(rows[R + rel]["assertions"] for rel in frag_rel)
assert root["assertions"] + frag_assert == orig["assertions"], (root["assertions"], frag_assert, orig["assertions"])
new_files = [dict(path=root_rel, sha256=root["sha256"], bytes=root["bytes"], assertions=root["assertions"] + meta["assertions"], functions=others)]
new_files += d["files"][1:]
for rel in frag_rel:
    r = rows[R + rel]
    new_files.append(dict(path=rel, sha256=r["sha256"], bytes=r["bytes"], assertions=r["assertions"], functions=per[rel]))
assert sum(f["assertions"] for f in new_files) == old_total
assert sum(1 for f in new_files for fn in f["functions"] if fn["law"]) == sum(1 for f in d["files"] for fn in f["functions"] if fn["law"]) == 22
d["files"] = new_files
sp = U + "🧬️surface-fixture.json"
schema = json.load(open(sp, encoding="utf-8"))
schema["properties"]["files"]["minItems"] = len(new_files)
schema["properties"]["files"]["maxItems"] = len(new_files)
print(f"functions {len(new_fns)}: text identical {same}, re-recorded after the include rebase {changed}; files {len(d['files'])}; assertions total {old_total} kept; laws 22 kept")
if apply:
    open(path, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    open(sp, "w", encoding="utf-8").write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
    print("written")
