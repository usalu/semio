"""🔭 Dump base->new diffs of staged batches with live status (BASE/NEW/DRIFT/NEWFILE/GONE)."""
import difflib, hashlib, json, pathlib, sys
C = pathlib.Path(__file__).resolve().parent.parent
LIVE = C.parents[6]
OUT = C / "🗑️generated" / "audit11"
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
def text(p):
    try: return p.read_text().splitlines(True) if p.exists() else []
    except UnicodeDecodeError: return ["<binary>\n"]
for d in sys.argv[1:]:
    s = C / d
    m = json.loads((s / "manifest.json").read_text())
    name = d.replace("-tools/staged/", "_").replace("/", "_")
    with open(OUT / f"{name}.diff", "w") as o:
        for rel, e in m.items():
            live = sha(LIVE / rel)
            st = "BASE" if live == e["base"] else "NEW" if live == e["new"] else "MISSING" if live is None else "DRIFT"
            print(f"{d} {st} b={'-' if e['base'] is None else 'y'} n={'-' if e['new'] is None else 'y'} {rel}")
            a = text(s / "base" / e["file"]); b = text(s / "files" / e["file"]) if e["new"] is not None else []
            o.write(f"######## {rel}  [{st}] base={'null' if e['base'] is None else 'y'} new={'null' if e['new'] is None else 'y'}\n")
            o.writelines(difflib.unified_diff(a, b, "base", "new", n=3))
