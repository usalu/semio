"""🪞 Rewrites chosen fields of a named struct into a `testkit`-gated public variant and a not-testkit variant that keeps the original visibility; every other byte of the file stays.
usage: dualfield.py <file> <Struct> <field,field,...>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ttcedit import replace
from ttcstub import mask

path, struct, names = sys.argv[1], sys.argv[2], set(sys.argv[3].split(","))
text = open(path, encoding="utf-8").read()
m = mask(text)
sm = re.search(r"\bstruct\s+" + re.escape(struct) + r"\s*(?:<[^{;]*>)?\s*\{", m)
assert sm and len(re.findall(r"\bstruct\s+" + re.escape(struct) + r"\b", m)) == 1, "struct not unique"
start = sm.end()
depth, i = 1, start
while depth:
    ch = m[i]
    if ch == "{":
        depth += 1
    elif ch == "}":
        depth -= 1
    i += 1
end = i - 1
segs, cur, d, a = [], start, 0, start
for k in range(start, end):
    ch = m[k]
    if ch in "<([{":
        d += 1
    elif ch in ">)]}":
        if ch == ">" and m[k - 1] == "-":
            continue
        d -= 1
    elif ch == "," and d == 0:
        segs.append((a, k))
        a = k + 1
segs.append((a, end))
out, last, done = [], start, set()
FIELD = re.compile(r"^(?P<lead>\s*)(?P<attrs>(?:#\[[^\]]*\]\s*)*)(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<name>[A-Za-z_]\w*)\s*:(?P<ty>.*)$", re.S)
for a, b in segs:
    seg = text[a:b]
    fm = FIELD.match(seg)
    if not fm or fm.group("name") not in names:
        continue
    if "testkit" in seg:
        raise SystemExit("already dual: " + fm.group("name"))
    vis = (fm.group("vis") or "").strip()
    ty = fm.group("ty").rstrip()
    trail = seg[len(seg.rstrip()):]
    attrs = fm.group("attrs")
    pubf = f'{attrs}#[cfg(feature = "testkit")] #[doc(hidden)] pub {fm.group("name")}:{ty},'
    privf = f'{attrs}#[cfg(not(feature = "testkit"))] {(vis + " ") if vis else ""}{fm.group("name")}:{ty}'
    new = "\n    ".join([pubf, privf])
    lead = fm.group("lead")
    out.append((a, b, (lead if "\n" in lead else lead.rstrip(" \t") + "\n    ") + new + trail))
    done.add(fm.group("name"))
assert done == names, (names - done)
res, last = [], 0
for a, b, rep in out:
    res.append(text[last:a])
    res.append(rep)
    last = b
res.append(text[last:])
new_text = "".join(res)
replace(path, text, new_text)
print(f"{struct}: {len(done)} fields doubled")
