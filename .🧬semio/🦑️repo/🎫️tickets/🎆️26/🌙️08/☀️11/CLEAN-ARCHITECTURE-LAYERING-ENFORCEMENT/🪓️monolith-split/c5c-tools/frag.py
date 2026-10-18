"""✂️ C5C include-fragment splitter for flat test roots (descendant of c4p47-tools/fragsplit.py + mkfrag.py): table of chunk starts -> verbatim depth-0 line ranges -> `<folder>/🦀️.rs` fragments that the root splices back with `include!("<folder>/🦀️.rs");`.
A fragment keeps the module identity of every item, so test paths, visibility and `use` scope are unchanged and nothing is widened.
Rules: chunk = from the first non-blank line after the previous item (leading `//#endregion` lines of the gap go to the PREVIOUS chunk) to the last line of the item before the next chunk; lines before the first chunk and lines after `end_line` stay in the root;
regions that span a cut are re-opened/closed with a balanced marker pair in each fragment; relative `include!`/`include_str!`/`include_bytes!` literals get one more `../`; a fragment must not contain a file `mod` declaration.
usage: frag.py <tag> [--stage]   (table: specs5.py)"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
sys.path.insert(0, HERE)
from rslex import lex, outline
from specs5 import FRAG

REPO = "/Users/ueli/Documents/semio"
STAGE = "/Users/ueli/.cache/c5c/stage-frag"
V = "️"
tag = sys.argv[1]
stage_it = "--stage" in sys.argv
spec = FRAG[tag]
files_tsv = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
rel = files_tsv[tag]
src = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
for old, new in spec.get("pre", []):
    assert src.count(old) == 1, (tag, "pre anchor", old)
    src = src.replace(old, new)
ls = src.split("\n")
if ls and ls[-1] == "":
    ls.pop()
depths, inlit = lex(src)
inner = spec.get("inner", False)
ind = "    " if inner else ""
base = 1 if inner else 0
if inner:
    last_line = max(k for k, l in enumerate(ls, 1) if l.strip())
    assert ls[0].rstrip().endswith("{") and ls[last_line - 1].strip() == "}", (tag, "inner module shape")
    items = outline("\n".join(ls[1:last_line - 1]) + "\n")
    for it in items:
        it["start"] += 1
        it["end"] += 1
else:
    items = outline(src)
RGN = re.compile(r"^\s*//\s?#region (.*)$")
ERG = re.compile(r"^\s*//\s?#endregion (.*)$")
DOCABLE = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|unsafe|const|default)\s+)*(?:fn|struct|enum|trait|impl|mod|const|static|type|union)\b")


def gap_split(a, b):
    """gap lines a..b (1-based inclusive, may be empty): returns (prev_part_end, next_part_start) where lines a..prev_part_end go to the previous chunk."""
    last = a - 1
    for k in range(a, b + 1):
        if ERG.match(ls[k - 1]):
            last = k
    return last, last + 1


def first_nonblank(a, b):
    for k in range(a, b + 1):
        if ls[k - 1].strip():
            return k
    return None


def last_nonblank(a, b):
    for k in range(b, a - 1, -1):
        if ls[k - 1].strip():
            return k
    return None


def item_at(line):
    for j, it in enumerate(items):
        if it["start"] + 1 == line:
            return j
    raise SystemExit(f"{tag}: line {line} is no item start; starts near: " + str([it['start'] + 1 for it in items if abs(it['start'] + 1 - line) < 6]))


gstack = []
_cur = []
for _l in ls:
    gstack.append(list(_cur))
    if RGN.match(_l):
        _cur.append(RGN.match(_l).group(1))
    elif ERG.match(_l) and _cur:
        _cur.pop()
starts = [item_at(r[0]) for r in spec["rows"]]
assert starts == sorted(set(starts)), (tag, "chunk starts not strictly increasing")
end_j = item_at(spec["end_line"]) - 1 if spec.get("end_line") else len(items) - 1
ranges = []
for ci, j in enumerate(starts):
    prev_end = items[j - 1]["end"] + 1 if j > 0 else 0
    gap_a, gap_b = prev_end + 1, items[j]["start"]
    _, nxt = gap_split(gap_a, gap_b)
    a = first_nonblank(nxt, items[j]["end"] + 1)
    if ci + 1 < len(starts):
        jn = starts[ci + 1]
        last_item_end = items[jn - 1]["end"] + 1
        pe, _ = gap_split(last_item_end + 1, items[jn]["start"])
        b = last_nonblank(last_item_end, max(pe, last_item_end))
    else:
        b = items[end_j]["end"] + 1
        pe, _ = gap_split(b + 1, (items[end_j + 1]["start"] if end_j + 1 < len(items) else len(ls)))
        b = last_nonblank(b, max(pe, b))
    ranges.append((a, b))
owner = {}
files = {}
report = {}
for (a, b), (first, emoji, kebab, doc) in zip(ranges, spec["rows"]):
    folder = emoji + V + kebab
    for k in range(a, b + 1):
        assert k not in owner, (tag, k)
        owner[k] = folder
    assert depths[a - 1] == base and not inlit[a - 1], (folder, a, "range start not at base depth")
    nxt_d = depths[b] if b < len(depths) else 0
    assert nxt_d == base and not (b < len(inlit) and inlit[b]), (folder, b, "range end not at base depth")
    start_stack = list(gstack[a - 1])
    st3 = list(start_stack)
    stray = []
    for l in ls[a - 1:b]:
        if RGN.match(l):
            st3.append(RGN.match(l).group(1))
        elif ERG.match(l):
            if st3:
                st3.pop()
            else:
                stray.append(ERG.match(l).group(1))
    need_open = start_stack + [n for n in stray]
    body = [f"//#region {n}" for n in need_open] + ls[a - 1:b] + [f"//#endregion {n}" for n in reversed(st3)]
    rebased = []
    for i, l in enumerate(body):
        assert not re.match(r"^\s*(?:#\[\s*path\b.*\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;", l), (folder, "file mod declaration inside a fragment", l)
        if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l) or re.search(r'include(?:_str|_bytes)?!\(\s*"\.\./', l):
            body[i] = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"', r'\1"../', l)
            rebased.append(l.strip()[:100])
    while body and not body[0].strip():
        body.pop(0)
    while body and not body[-1].strip():
        body.pop()
    firstl = next(l for l in body if l.strip())
    k = 0
    while body[k].lstrip().startswith(("#[", "///", "//!")) and not body[k].lstrip().startswith("//#"):
        k += 1
    head_line = next((l.strip() for l in body if l.strip() and not l.lstrip().startswith(("#[", "///", "//"))), "")
    marker = "///" if (not RGN.match(firstl) and not ERG.match(firstl) and not firstl.lstrip().startswith("// ") and DOCABLE.match(head_line)) else "//"
    files[folder] = f"{ind}{marker} {emoji}{V} {doc}" + "\n\n" + "\n".join(body) + "\n"
    report[folder] = dict(lines=len(body) + 2, range=(a, b), marker=marker, rebased=len(rebased), regions_opened=need_open, regions_closed=[n for n in reversed(st3)], stray=stray)
parent = []
placed = set()
for k in range(1, len(ls) + 1):
    f = owner.get(k)
    if f is None and placed and not ls[k - 1].strip():
        continue
    if f is not None:
        if f not in placed:
            placed.add(f)
            if parent and parent[-1].strip() and not parent[-1].lstrip().startswith("include!(") and parent[-1].strip() != "{" and not parent[-1].rstrip().endswith("{"):
                parent.append("")
            parent.append(f'{ind}include!("{f}/🦀️.rs");')
        continue
    parent.append(ls[k - 1])
tidy = []
for l in parent:
    if l.strip() == "" and tidy and tidy[-1].strip() == "":
        continue
    tidy.append(l)
if spec.get("root_doc"):
    tidy = [spec["root_doc"], ""] + tidy
root_text = "\n".join(tidy).rstrip("\n") + "\n"
new_lines = root_text.split("\n") + [l for t in files.values() for l in t.split("\n")]
if spec.get("root_doc"):
    new_lines.remove(spec["root_doc"])
c = lambda xs: collections.Counter(l.strip() for l in xs if l.strip())
gone = c(ls) - c(new_lines)
added = c(new_lines) - c(ls)
print(f"[{tag}] root {len(root_text.split(chr(10))) - 1} lines; fragments {len(files)}: " + json.dumps({k: v['lines'] for k, v in report.items()}, ensure_ascii=False))
print("  REMOVED (rebased literals only expected):", sum(gone.values()), "ADDED:", sum(added.values()))
for f, r in report.items():
    extra = []
    if r["regions_opened"] or r["regions_closed"]:
        extra.append(f"reopen={r['regions_opened']} close={r['regions_closed']}")
    print("  ", f, r["range"], r["lines"], r["marker"], f"rebased={r['rebased']}", *extra)
if stage_it:
    bdir = os.path.dirname(rel)
    out = {rel: root_text}
    for folder, txt in files.items():
        out[f"{bdir}/{folder}/🦀️.rs"] = txt
    for r_, txt in out.items():
        p = os.path.join(STAGE, r_)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write(txt)
    print("  staged", len(out), "files to", STAGE)
    json.dump(dict(tag=tag, file=rel, root=rel, folders=list(files)), open(f"{HERE}/staged-{tag}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
