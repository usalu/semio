"""🧾 C5F re-record of the plugin units preservation records after the include-fragment cuts (laws keep name and body; only the relative include literals of a fragment gained one `../`).
Every re-recorded sha is PROVEN: the new function text with the `../` rebase reversed must equal the old recorded text (old sha equal to the recorded one) before the record is rewritten.
usage: rerecord.py [--apply]"""
import json, os, re, sys, hashlib, collections
HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.dirname(HERE)
G = F + "/🗑️generated/"
R = "/Users/ueli/Documents/semio/"
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
U = R + PL + "🧪️tests/📦️units/"
apply = "--apply" in sys.argv
sha = lambda t: hashlib.sha256(t.encode("utf-8")).hexdigest()
unrebase = lambda t: re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', t)
rebased_ok = lambda new, old: unrebase(new) == old
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
frag = json.load(open(G + "c5f-fnrows-frag-text.json", encoding="utf-8"))
orig = json.load(open(G + "c5f-fnrows-orig-text.json", encoding="utf-8"))
# ---- preserved.json
pres_path = U + "📋️preserved.json"
pres = json.load(open(pres_path, encoding="utf-8"))
tag_of_root = {PL + os.path.relpath(R + ft[t], R + PL): t for t in ft}
tag_of_root = {ft[t]: t for t in ft}
old_text = {}
for t in ["c3", "c4", "c5", "c6", "c7"]:
    rows = orig[HERE + f"/orig/{t}.rs"]["fns"]
    for r in rows:
        if r["test"]:
            old_text[(t, r["name"])] = r
new_by_name = collections.defaultdict(list)
for path, d in frag.items():
    t = next(t for t in ["c3", "c4", "c5", "c6", "c7"] if os.path.dirname(R + ft[t]) in path)
    for r in d["fns"]:
        if r["test"]:
            new_by_name[(t, r["name"])].append((path, r))
changed = same = moved = 0
errors = []
per_file_assert = collections.Counter()
for law in pres["laws"]:
    t = tag_of_root.get(law["source"])
    if t not in ["c3", "c4", "c5", "c6", "c7"]:
        continue
    key = (t, law["name"])
    cands = new_by_name.get(key, [])
    if len(cands) != 1:
        errors.append(("candidates", key, len(cands))); continue
    path, nr = cands[0]
    orow = old_text.get(key)
    if orow is None or orow["sha256"] != law["sha256"]:
        errors.append(("old sha differs from the record", key)); continue
    if nr["assertions"] != law["assertions"]:
        errors.append(("assertions differ", key)); continue
    if nr["sha256"] == law["sha256"]:
        same += 1
    elif rebased_ok(nr["text"], orow["text"]):
        law["sha256"] = nr["sha256"]; changed += 1
    else:
        errors.append(("text differs beyond the rebase", key)); continue
    law["source"] = os.path.relpath(path, R)
    moved += 1
print(f"preserved.json: laws moved {moved}, sha unchanged {same}, sha re-recorded (rebase proven) {changed}, errors {len(errors)}")
for e in errors[:20]:
    print("  ERROR", e)
# assertion accounting as the units law computes it
sources = {pres["original"], *[l["source"] for l in pres["laws"]]}
fam = {R + k: v for k, v in json.load(open(G + "c5f-fnrows-family-tree.json", encoding="utf-8")).items()}
ct_root = R + ft["ct"]
total = 0
missing = []
for s in sorted(sources):
    p = R + s
    if p == ct_root:
        total += fam[p]["assertions"]; continue
    d = frag.get(p)
    if d is None:
        extra = json.load(open(G + "c5f-fnrows-other.json", encoding="utf-8")) if os.path.exists(G + "c5f-fnrows-other.json") else {}
        d = extra.get(R + s) or extra.get(s)
    if d is None:
        missing.append(s); continue
    total += d["assertions"]
print("assertion total over the new source set:", total, "(sources not measured:", len(missing), ") expected", pres["originalAssertions"] + pres["addedAssertions"])
for m in missing[:12]:
    print("  unmeasured", m.replace(PL, "PLUG/"))
# ---- surface-carriers remaining
sc_path = U + "📋️surface-carriers.json"
sc = json.load(open(sc_path, encoding="utf-8"))
oldms = collections.Counter((r["name"], r["sha256"]) for r in sc["remaining"])
newrows = []
for p, d in fam.items():
    for r in d["fns"]:
        newrows.append(r)
mapped = collections.Counter()
upd = {}
bad = []
for r in newrows:
    k = (r["name"], r["sha256"])
    if oldms[k] > mapped[k]:
        mapped[k] += 1; continue
    rk = (r["name"], sha(unrebase(r["text"])))
    if oldms[rk] > mapped[rk]:
        mapped[rk] += 1; upd.setdefault(rk, []).append(r); continue
    bad.append(k)
print(f"surface-carriers.json: new family functions {len(newrows)}, old remaining {len(sc['remaining'])}, mapped unchanged {sum(mapped.values()) - sum(len(v) for v in upd.values())}, re-recorded {sum(len(v) for v in upd.values())}, unmatched new {len(bad)}, unmatched old {sum((oldms - mapped).values())}")
if apply and not errors and not bad and sum((oldms - mapped).values()) == 0:
    for row in sc["remaining"]:
        k = (row["name"], row["sha256"])
        if k in upd and upd[k]:
            nr = upd[k].pop(0)
            row["sha256"] = nr["sha256"]; row["bytes"] = nr["bytes"]
    open(sc_path, "w", encoding="utf-8").write(json.dumps(sc, indent=2, ensure_ascii=False) + "\n")
    open(pres_path, "w", encoding="utf-8").write(json.dumps(pres, indent=2, ensure_ascii=False) + "\n")
    print("written preserved.json and surface-carriers.json")
