import glob, os, re, sys

orig = "/Users/ueli/.cache/c5h/orig/diff"
now = sys.argv[1]


def functions(text, start=0):
    return re.findall(r"^export function parse\w+\(.*?^\}\n", text[start:], re.M | re.S)


otext = open(os.path.join(orig, "🟦️.ts"), encoding="utf-8").read()
region = otext.index("//#region 🚪️Parsers")
before = functions(otext, region)
after = []
for path in sorted(glob.glob(now + "/**/🟦️.ts", recursive=True)):
    if "🧪️tests" in path:
        continue
    text = open(path, encoding="utf-8").read()
    start = text.find("//#region 🚪️Parsers")
    if start >= 0:
        after += functions(text, start)
print("parsers before", len(before), "after", len(after), "multiset equal", sorted(before) == sorted(after))
for child in ["📋️deltas-requirements", "🗃️deltas-records", "🧱️deltas-program-structure", "🧾️program-diff", "🩹️patch-types"]:
    a = open(os.path.join(orig, child, "🟦️.ts"), encoding="utf-8").read()
    b = open(os.path.join(now, child, "🟦️.ts"), encoding="utf-8").read()
    print(child, "unchanged" if a == b else "CHANGED")
# leaf: all non-import lines of the original outside moved functions remain
leaf = open(os.path.join(now, "🟦️.ts"), encoding="utf-8").read()
rest = otext
for fn in before:
    rest = rest.replace(fn + "\n", "", 1)
olines = [l for l in rest.splitlines() if not l.startswith("import ") and l.strip()]
nlines = set(l for l in leaf.splitlines())
print("original leaf non-import lines missing in new leaf:", [l[:80] for l in olines if l not in nlines])
extra = [l[:90] for l in leaf.splitlines() if l.strip() and l not in set(otext.splitlines())]
print("new leaf lines not in original:", len(extra))
for l in extra: print("  +", l)
