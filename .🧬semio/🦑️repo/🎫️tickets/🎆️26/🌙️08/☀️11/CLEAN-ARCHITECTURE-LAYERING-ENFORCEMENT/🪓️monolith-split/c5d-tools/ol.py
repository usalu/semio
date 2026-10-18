"""📑 C5D item outline: depth-0 non-use items of a snapshot (start incl. docs/attrs, end, kind, name, test flag) plus region markers. usage: ol.py <tag> [--uses]"""
import sys, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
from rslex import outline
tag = sys.argv[1]
text = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
ls = text.split("\n")
off = 0
if tag == "atc":
    last = max(k for k, l in enumerate(ls, 1) if l.strip())
    its = outline("\n".join(ls[1:last - 1]) + "\n")
    for it in its:
        it["start"] += 1
        it["end"] += 1
else:
    its = outline(text)
for it in its:
    s, e = it["start"] + 1, it["end"] + 1
    if it["kind"] == "use" and "--uses" not in sys.argv:
        continue
    attrs = " ".join(l.strip() for l in ls[it["start"]:it["end"] + 1][:6] if l.strip().startswith("#["))
    t = "T" if re.search(r"#\[\s*(?:[\w:]*::)?(test|async_test|tokio::test)\b", attrs) else " "
    print(f"{s:5}-{e:<5} {e-s+1:4} {t} {it['kind']:7} {(it['name'] or it['head'] or '')[:70]}")
for i, l in enumerate(ls):
    if re.match(r"\s*//\s?#(end)?region", l):
        print(f"{i+1:5} REGION {l.strip()[:80]}")
