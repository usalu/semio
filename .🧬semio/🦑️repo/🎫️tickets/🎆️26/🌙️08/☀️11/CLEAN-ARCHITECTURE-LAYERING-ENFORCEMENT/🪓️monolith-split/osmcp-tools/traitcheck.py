import sys, os, re, collections
sys.path.insert(0, ".")
from layout import *
lay = Layout("plan3")
w = lay.w
# local traits and their method names
traits = {}
for a in w.atoms:
    if a.kind == "trait" and id(a) in lay.crate_of:
        traits[a.name] = set(re.findall(r'\bfn\s+([A-Za-z_]\w*)', a.item.text))
inherent = set()
for a in w.atoms:
    for it in [a.item] + a.impls:
        if it.kind == "impl" and not it.impl_trait:
            inherent |= set(re.findall(r'\bfn\s+([A-Za-z_]\w*)', it.text))
        if it.kind == "struct" or it.kind == "enum": pass
# free fns are not methods
COMMON = set("read write list call get len now_ms append next send lock insert remove take push poll flush close open run drop new default from into clone".split())
uniq = {t: {m for m in ms if m not in inherent and m not in COMMON} for t, ms in traits.items()}
print({t: sorted(ms)[:6] for t, ms in uniq.items() if ms})
S = "/Users/ueli/Documents/semio/"
M = S + "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
bad = 0
for dp, dn, fn in os.walk(M):
    if "🤖️generated" in dp: continue
    for f in fn:
        if not f.endswith(".rs"): continue
        p = os.path.join(dp, f)
        t = strip_code(open(p, encoding="utf-8").read())
        for tr, ms in uniq.items():
            if not ms: continue
            calls = [m for m in ms if re.search(r'\.\s*' + re.escape(m) + r'\s*(?:::<[^>]*>)?\s*\(', t)]
            if calls and not re.search(r'(?<![\w])' + re.escape(tr) + r'(?![\w])', t):
                bad += 1; print("TRAIT NOT IN SCOPE?", os.path.relpath(p, M), tr, calls[:3])
print("suspects", bad)
