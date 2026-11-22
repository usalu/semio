"""📦️ Lists dist-exporting packages without artifactPackage metadata and sources that import their own package's dist."""
import json, os, re, sys
pkgs = {}
for top in sys.argv[1:]:
    for d, ds, fs in os.walk(top):
        ds[:] = [x for x in ds if x not in ("node_modules", "dist", "target", ".git", "🕸️bindings") and not x.startswith(".")]
        if "package.json" in fs and "📋️project.json" in fs:
            try: p = json.load(open(os.path.join(d, "package.json"), encoding="utf-8")); j = json.load(open(os.path.join(d, "📋️project.json"), encoding="utf-8"))
            except Exception: continue
            if "./dist/" in json.dumps(p.get("exports", {}), ensure_ascii=False):
                meta = j.get("metadata", {}).get("semio", {}).get("artifactPackage")
                pkgs[p["name"]] = (d, meta)
for name, (d, meta) in sorted(pkgs.items()):
    if not meta: print("NO-META", name, d)
    src = os.path.normpath(os.path.join(d, meta["source"])) if meta else d
    root = os.path.dirname(src) if meta else d
    rx = re.compile(r'''from\s*["']''' + re.escape(name) + r'''["']''')
    for dd, ds, fs in os.walk(d):
        ds[:] = [x for x in ds if x not in ("node_modules", "dist")]
        for f in fs:
            if f.endswith(".ts") and rx.search(open(os.path.join(dd, f), encoding="utf-8", errors="ignore").read()): print("SELF", name, os.path.join(dd, f))
