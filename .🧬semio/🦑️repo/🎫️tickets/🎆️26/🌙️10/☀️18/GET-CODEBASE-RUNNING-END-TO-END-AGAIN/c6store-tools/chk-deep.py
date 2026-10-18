"""✅ C6STORE two-level include-fragment check: expands the include! lines of the live root recursively (fragment first doc line dropped, one `../` rebase reversed per level) and compares the line sequence (region markers, blanks and indentation ignored) and the #[test] multiset with the snapshot. usage: chk-deep.py <tag>"""
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio/"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
tag = sys.argv[1]
INCL = re.compile(r'^\s*include!\("([^"/]+)/🦀️\.rs"\);$')
REG = re.compile(r"^\s*//\s?#(end)?region ")
TESTRX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
def expand(path, depth=0):
    base = os.path.dirname(path)
    out = []
    for line in open(path, encoding="utf-8").read().split("\n"):
        m = INCL.match(line)
        if m and os.path.isfile(os.path.join(base, m.group(1), "🦀️.rs")):
            sub = expand(os.path.join(base, m.group(1), "🦀️.rs"), depth + 1)
            sub = sub[1:]
            sub = [re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', l, count=1) for l in sub]
            out.extend(sub)
        else:
            out.append(line)
    return out
live = expand(R + ft[tag])
orig = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read().split("\n")
norm = lambda ls: [l.strip() for l in ls if l.strip() and not REG.match(l)]
a, b = norm(orig), [l for l in norm(live) if not (l.startswith("//") and not l.startswith("///") and not l.startswith("//!"))]
a2 = [l for l in a if not (l.startswith("//") and not l.startswith("///") and not l.startswith("//!"))]
ok = a2 == b
if not ok:
    k = next((i for i, (x, y) in enumerate(zip(a2, b)) if x != y), min(len(a2), len(b)))
    print("sequence differs at", k, a2[k:k+1], b[k:k+1])
ot = sorted(m.group(1) for m in TESTRX.finditer("\n".join(orig)))
nt = sorted(m.group(1) for m in TESTRX.finditer("\n".join(live)))
print(f"[{tag}] deep sequence {'EQUAL' if ok else 'DIFFERS'} lines {len(a2)}/{len(b)} tests {len(ot)}->{len(nt)} {'OK' if ok and ot == nt else 'PROBLEM'}")
