#!/usr/bin/env python3
"""FORMS-V4: lists (and with --write declares) the geometric case inputs (length, angle units) that no geometry role draws, per code.
Usage: scan.py [--write]"""
import json, re, sys, collections
SRC = open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/catalogue-bridge-v5/geometry_gen.py").read()
SRC = SRC[:SRC.rindex("\nmain()")]
G = {}
exec(SRC, G)
LEN = ("Metre", "Millimetre")
ANGLE = ("Degree", "Radian")
SKIP = re.compile(r"deflect|displac|settle|crack|wave|rough|amplitude|aggregate|grain.?size|toler|slip(?! circle)|stroke|travel|rise.?time|elong|shrink|creep|drift|sag|camber|vibrat|height of wave|period", re.I)
Z = re.compile(r"height|depth|thick|level|elev|cover|head|draught|embed|rise|settle|storey|story", re.I)
Y = re.compile(r"width|edge|lateral|breadth|offset|across|transverse|side|eccentric|gap|clear", re.I)
GRIPPED = {("T-90", k) for k in ("h1", "h2", "h3", "s1", "s2", "s3")} | {("T-94", "s"), ("T-94", "h"), ("T-96", "spacing"), ("T-96", "member_b"), ("T-96", "member_h"), ("T-122", "bar_spacing")} | {("T-95", k) for k in ("span", "height", "chord_b", "chord_h", "web_b", "web_h")}
write = "--write" in sys.argv
stats = collections.Counter()
for code, (name, rel) in G["CASES"].items():
    directory = f'{G["ROOT"]}/{code}/{rel}/📐️geometry'
    path = f"{directory}/🔣️.json"
    text = open(path).read()
    source = json.loads(text)
    titles = {}
    for dp, _, files in G["os"].walk(f'{G["ROOT"]}/{code}'):
        if "inputs" in dp and "tests" not in dp:
            for f in files:
                if f.endswith(".rs"):
                    for m in re.finditer(r'InputSpec::number\("([^"]+)",\s*"((?:[^"\\]|\\.)*)"', open(G["os"].path.join(dp, f)).read()):
                        titles.setdefault(m.group(1), m.group(2))
    entries, vocab = G["extract_entries"](code), G["extract_vocabulary"](code)
    def spec(eid, key):
        found = [r for r in vocab if r["key"] == key and (not r["scope"] or eid in r["scope"])]
        scoped = [r for r in found if eid in r["scope"]]
        return scoped[0] if scoped else (found[0] if found else None)
    labels = {}
    added = skipped = 0
    for eid, row in source["entries"].items():
        if eid not in entries:
            continue
        used = {row["width"], row["length"], row["height"], row["load"], *row["aux"].values()} - {""}
        series = {v for r, v in row["aux"].items() if r in ("layers", "plies", "ply_dir")}
        taken = {}
        for key in entries[eid][1]:
            sp = spec(eid, key)
            if sp is None or sp["kind"] != "number" or key in used or sp["unit"] not in LEN + ANGLE or (eid, key) in GRIPPED:
                continue
            if any(re.fullmatch(re.escape(re.sub(r"\d+$", "", s)) + r"\d+", key) for s in series):
                continue
            title = titles.get(key, key) + " " + key
            if sp["unit"] in ANGLE:
                stem = "dim_angle"
            elif SKIP.search(title):
                skipped += 1
                print(code, eid, key, "SKIPPED", title, file=sys.stderr)
                continue
            else:
                stem = "dim_z" if Z.search(title) else "dim_y" if Y.search(title) else "dim_x"
            n = taken.get(stem, 0) + 1
            taken[stem] = n
            role = stem + ("" if n == 1 else str(n))
            stats[stem] += 1
            added += 1
            print(code, eid, key, sp["unit"], "->", role)
            if write:
                row["aux"][role] = key
    print(code, "declared", added, "skipped", skipped, file=sys.stderr)
    if write:
        open(path, "w").write(json.dumps(source, ensure_ascii=False, indent=1) + ("\n" if text.endswith("\n") else ""))
print(dict(stats), file=sys.stderr)
