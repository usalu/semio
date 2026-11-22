"""🕸️ Lists relative TS/JS imports whose target is missing and not under any generatorContracts output root."""
import json, os, re, sys
tax = json.load(open("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", encoding="utf-8"))
roots = [r["path"] for c in tax["generatorContracts"].values() for r in c.get("outputRoots", [])]
pat = re.compile(r'''(?:from\s*|import\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
for top in sys.argv[1:]:
    for d, ds, fs in os.walk(top):
        ds[:] = [x for x in ds if x not in ("node_modules", "dist", "target", ".git") and not x.startswith(".🧾")]
        for f in fs:
            if not re.search(r"\.(ts|tsx|mts|mjs|js)$", f) or f.endswith(".d.ts"): continue
            p = os.path.join(d, f)
            if "🕸️bindings" in p or "🤖️generated" in p: continue
            try: s = open(p, encoding="utf-8").read()
            except Exception: continue
            for m in pat.finditer(s):
                spec = m.group(1)
                if "${" in spec: continue
                t = os.path.normpath(os.path.join(d, spec))
                if os.path.exists(t) or os.path.exists(t + ".ts"): continue
                if any(t == r or t.startswith(r + "/") for r in roots): continue
                print(p, "->", spec)
