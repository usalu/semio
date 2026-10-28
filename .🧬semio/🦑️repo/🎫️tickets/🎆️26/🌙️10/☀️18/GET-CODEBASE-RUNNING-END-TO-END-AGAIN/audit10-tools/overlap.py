"""🧷 Cross-check staged manifests: shared rows, stacking (base == other.new) and live drift."""
import hashlib, json, pathlib, sys
C = pathlib.Path(__file__).resolve().parent.parent
LIVE = C.parents[6]
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
ms = {d: json.loads((C / d / "manifest.json").read_text()) for d in sys.argv[1:]}
rows = {}
for d, m in ms.items():
    for rel, e in m.items():
        rows.setdefault(rel, []).append((d, e))
for rel, es in rows.items():
    live = sha(LIVE / rel)
    tag = lambda h: "LIVE" if h == live else (h or "none")[:8]
    if len(es) > 1 or any(e["base"] != live for _, e in es):
        print(f"## {rel[-100:]}  live={(live or 'none')[:8]}")
        for d, e in es:
            stk = [o for o, oe in es if o != d and oe["new"] == e["base"]]
            print(f"   {d}: base={tag(e['base'])} new={tag(e['new'])}" + (f" STACKED-ON {stk}" if stk else ""))
