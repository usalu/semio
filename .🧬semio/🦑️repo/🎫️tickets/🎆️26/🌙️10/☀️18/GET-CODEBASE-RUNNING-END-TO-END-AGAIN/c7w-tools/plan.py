"""🗺️ c7w planner: plan.py <tag> prints the compact item outline (non-use items), the region markers, and for every impl/mod over 250 lines the depth-1 method boundaries."""
import re, sys, os, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools')
from rslex import outline
tag = sys.argv[1]
text = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
ls = text.split("\n")
its = outline(text)
big = []
print("lines", len(ls))
for it in its:
    s, e = it["start"] + 1, it["end"] + 1
    if it["kind"] == "use":
        continue
    attrs = " ".join(l.strip() for l in ls[it["start"]:it["end"] + 1][:4] if l.strip().startswith("#["))
    n = e - s + 1
    nm = (it['name'] or it['head'] or '')[:60]
    print(f"{s:5}-{e:<5} {n:4} {it['kind']:6} {nm}  {attrs[:50] if n > 3 or 'cfg' in attrs else ''}")
    if n > 250 and it["kind"] in ("impl", "mod", "fn"):
        big.append((s, e, it['kind'], nm))
for i, l in enumerate(ls):
    if re.match(r"\s*//\s?#(end)?region", l):
        print(f"{i+1:5} REGION {l.strip()[:80]}")
for s, e, k, nm in big:
    print("== methods of", k, nm, s, e)
    if k == 'mod':
        continue
    print(subprocess.run([sys.executable, HERE + "/methods.py", tag, str(s), str(e)], capture_output=True, text=True).stdout)
