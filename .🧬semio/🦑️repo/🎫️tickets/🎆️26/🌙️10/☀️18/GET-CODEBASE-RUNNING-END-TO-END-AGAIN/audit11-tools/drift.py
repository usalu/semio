"""🧭 Shows live-vs-base diffs for drifted manifest rows: python3 -I drift.py <batchDir>..."""
import difflib, hashlib, json, pathlib, sys
C = pathlib.Path(__file__).resolve().parent.parent
LIVE = C.parents[6]
for d in sys.argv[1:]:
    s = C / d; m = json.loads((s / "manifest.json").read_text())
    for rel, e in m.items():
        p = LIVE / rel
        if not p.exists() or e["base"] is None: continue
        h = hashlib.sha256(p.read_bytes()).hexdigest()
        if h in (e["base"], e["new"]): continue
        print(f"######## DRIFT {d} {rel}")
        sys.stdout.writelines(difflib.unified_diff((s/"base"/e["file"]).read_text().splitlines(True), p.read_text().splitlines(True), "base", "live", n=1))
