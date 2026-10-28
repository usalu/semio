"""🔬 Dump base->new diffs of staged batches and their live status (base/new/drift)."""
import difflib, hashlib, json, pathlib, sys
C = pathlib.Path(__file__).resolve().parent.parent
LIVE = C.parents[6]
OUT = C / "🗑️generated" / "audit8"
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
for d in sys.argv[1:]:
    s = C / d
    m = json.loads((s / "manifest.json").read_text())
    name = d.replace("-tools/staged/", "_").replace("/", "_")
    with open(OUT / f"{name}.diff", "w") as o:
        for rel, e in m.items():
            live = sha(LIVE / rel)
            st = "BASE" if live == e["base"] else "NEW" if live == e["new"] else "MISSING" if live is None else "DRIFT"
            if e["base"] is None and live is None: st = "NEWFILE"
            print(f"{d} {st} {rel[-90:]}")
            bp = s / "base" / e["file"]
            a = bp.read_text().splitlines(True) if bp.exists() else []
            b = (s / "files" / e["file"]).read_text().splitlines(True)
            o.write(f"######## {rel}  [{st}]\n")
            o.writelines(difflib.unified_diff(a, b, "base", "new", n=3))
