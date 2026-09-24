"""📦️ Maps changed repo paths to the Cargo packages that compile them (longest lib-source-dir prefix over every Cargo.toml)."""
import os, re, sys, collections
root = "/Users/ueli/Documents/semio/"
libs = []
for base in ("✏️s", "🧰️framework", "🌎️hub", "🏢️semio-tech"):
    for dp, ds, fs in os.walk(root + base):
        ds[:] = [d for d in ds if d not in ("node_modules", "target", "dist", "🗑️generated", ".git")]
        if "Cargo.toml" not in fs: continue
        t = open(os.path.join(dp, "Cargo.toml"), encoding="utf-8").read()
        m = re.search(r'^\[package\]\s*\n(?:[^\[].*\n)*?name\s*=\s*"([^"]+)"', t, re.M)
        if not m: continue
        lm = re.search(r'^\[lib\]\s*\n(?:[^\[].*\n)*?path\s*=\s*"([^"]+)"', t, re.M)
        src = os.path.normpath(os.path.join(dp, lm.group(1) if lm else "src/lib.rs"))
        libs.append((os.path.dirname(src) + "/", m.group(1)))
libs.sort(key=lambda x: -len(x[0]))
paths = [l.strip() for f in sys.argv[1:] for l in open(f, encoding="utf-8") if l.strip()]
pkgs, missing = collections.Counter(), []
for p in paths:
    if p.startswith(".tmp-ticket") or p.endswith(".ts"): continue
    full = root + p
    hit = next((n for d, n in libs if full.startswith(d)), None)
    if hit: pkgs[hit] += 1
    else: missing.append(p)
for k, v in sorted(pkgs.items()): print(v, k)
print("no package:", len(missing)); [print("  ", m) for m in missing[:30]]
