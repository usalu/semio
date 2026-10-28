"""🔢 For each schemafirst batch: numeric literals (>=100) a code row loses, and whether the batch's new schema values hold each one: python3 -I moved_values.py <batchDir>..."""
import json, pathlib, re, sys, collections
C = pathlib.Path(__file__).resolve().parent.parent
LIVE = C.parents[6]
num = re.compile(r"(?<![\w.$])(0x[0-9a-fA-F_]+|\d[\d_]*(?:\.\d+)?(?:e\d+)?)(?![\w.])")
def nums(text):
    out = collections.Counter()
    for m in num.finditer(text):
        t = m.group(1).replace("_", "")
        try: v = int(t, 16) if t.startswith("0x") else float(t)
        except ValueError: continue
        if v >= 100: out[v] += 1
    return out
def jvals(o, acc):
    if isinstance(o, bool): return
    if isinstance(o, (int, float)): acc.add(float(o))
    elif isinstance(o, dict): [jvals(v, acc) for v in o.values()]
    elif isinstance(o, list): [jvals(v, acc) for v in o]
allvals = set()
for p in (LIVE / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build").rglob("🧬️schema/*.json"):
    if "🧪️tests" in str(p): continue
    try: jvals(json.loads(p.read_text()), allvals)
    except Exception: pass
for d in sys.argv[1:]:
    s = C / d; m = json.loads((s / "manifest.json").read_text())
    vals = set(allvals)
    for rel, e in m.items():
        if rel.endswith(".json") and e["new"] is not None: jvals(json.loads((s / "files" / e["file"]).read_text()), vals)
    for rel, e in m.items():
        if rel.endswith(".json") or e["base"] is None or e["new"] is None or "🧪️tests" in rel: continue
        a = nums((s / "base" / e["file"]).read_text()); b = nums((s / "files" / e["file"]).read_text())
        lost = {v: a[v] - b.get(v, 0) for v in a if a[v] > b.get(v, 0)}
        miss = {v: n for v, n in lost.items() if v not in vals}
        print(f"{d.split('/')[-1]:13} lost={sum(lost.values()):3} unmatched={dict(miss) if miss else '-'} {rel.split('📚️library/')[-1][-70:]}")
