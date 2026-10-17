"""✂️ C4P43 include-fragment splitter for flat test target roots (adapted from rf8-tools/fragsplit.py, which is tied to its own target table).
Cuts verbatim depth-0 line ranges of the ORIGINAL snapshot into `<folder>/🦀️.rs` fragments that the root splices back with `include!("<folder>/🦀️.rs");`.
A fragment keeps the module identity of every item, so test paths, visibility and `use` scope are unchanged and nothing is widened.
Rebased on cut: relative `include!`/`include_str!`/`include_bytes!` literals get one more `../`.
usage: fragsplit.py <spec.json> [--stage]
spec: {file (repo-relative), original, stage, chunks:[{folder, doc, ranges:[[a,b]...], pre?:[lines], post?:[lines], at?:line, replace?:[[from,to]...]}]}"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools")
from rslex import lex, outline, multiline_literal_lines

REPO = "/Users/ueli/Documents/semio"
DOCABLE = {"fn", "struct", "enum", "trait", "impl", "mod", "const", "static", "type", "union"}
spec = json.load(open(sys.argv[1], encoding="utf-8"))
stage_it = "--stage" in sys.argv
src = open(spec["original"], encoding="utf-8").read()
ls = src.split("\n")
if ls and ls[-1] == "":
    ls.pop()
assert not multiline_literal_lines(src), "multi-line literals present"
depths, inlit = lex(src)
owner = {}
anchors = {}
skip = set()
files = {}
report = {}
for c in spec["chunks"]:
    body = []
    for a, b in c["ranges"]:
        for k in range(a, b + 1):
            assert k not in owner, f"line {k} claimed by {owner[k]} and {c['folder']}"
            owner[k] = c["folder"]
        assert depths[a - 1] == 0 and not inlit[a - 1], (c["folder"], a, "range does not start at depth 0")
        nxt = depths[b] if b < len(depths) else 0
        assert nxt == 0 and not (b < len(inlit) and inlit[b]), (c["folder"], b, "range does not end at depth 0")
        if body:
            body.append("")
        body.extend(ls[a - 1:b])
        skip.update(range(a, b + 1))
    body = list(c.get("pre", [])) + body + list(c.get("post", []))
    replaced = []
    for frm, to in c.get("replace", []):
        joined = "\n".join(body)
        assert frm in joined, (c["folder"], "replace anchor missing", frm)
        replaced.append((joined.count(frm), frm[:90], to[:90]))
        body = joined.replace(frm, to).split("\n")
    rebased = []
    for i, l in enumerate(body):
        if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l):
            body[i] = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"([^"/])', r'\1"../\2', l)
            rebased.append(l.strip()[:100])
    while body and body[0].strip() == "":
        body.pop(0)
    while body and body[-1].strip() == "":
        body.pop()
    last = next((l for l in reversed(body) if l.strip()), "")
    assert not last.strip().startswith("///") and not last.strip().startswith("#["), (c["folder"], "fragment ends with an orphan doc or attribute", last)
    its = outline("\n".join(body) + "\n")
    first = its[0] if its else None
    marker = "///" if first and first["kind"] in DOCABLE and not c.get("pre") else "//"
    head = f"{marker} {c['doc']}"
    files[c["folder"]] = head + "\n\n" + "\n".join(body) + "\n"
    anchors[c.get("at") or c["ranges"][0][0]] = c
    report[c["folder"]] = dict(replaced=replaced, lines=len(body) + 2, first=(first["kind"], first["name"]) if first else None, marker=marker, rebased=rebased)
parent = []
for k in range(1, len(ls) + 1):
    if k in anchors:
        if parent and parent[-1].strip() and not parent[-1].startswith("include!("):
            parent.append("")
        parent.append(f'include!("{anchors[k]["folder"]}/🦀️.rs");')
    if k in skip:
        continue
    if parent and parent[-1].startswith("include!(") and ls[k - 1].strip():
        parent.append("")
    parent.append(ls[k - 1])
tidy = []
for l in parent:
    if l.strip() == "" and tidy and tidy[-1].strip() == "":
        continue
    tidy.append(l)
root_text = "\n".join(tidy).rstrip("\n") + "\n"


def multiset(lines):
    return collections.Counter(l.strip() for l in lines if l.strip() != "")


new_lines = root_text.split("\n") + [l for t in files.values() for l in t.split("\n")]
gone = multiset(ls) - multiset(new_lines)
added = multiset(new_lines) - multiset(ls)
print("root", len(root_text.split("\n")) - 1, "lines; fragments:", {k: len(v.split("\n")) - 1 for k, v in files.items()})
print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:1800])
print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:1800])
for f, r in report.items():
    print(f, r)
if stage_it:
    fileabs = os.path.join(REPO, spec["file"])
    base = os.path.dirname(spec["file"])
    out = {spec["file"]: root_text}
    for folder, txt in files.items():
        out[f"{base}/{folder}/🦀️.rs"] = txt
    for rel, txt in out.items():
        p = os.path.join(spec["stage"], rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write(txt)
    print("staged", len(out), "files to", spec["stage"])
