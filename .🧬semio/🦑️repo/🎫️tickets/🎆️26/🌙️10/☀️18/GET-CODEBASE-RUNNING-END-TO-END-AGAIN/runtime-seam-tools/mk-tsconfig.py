#!/usr/bin/env python3
"""Writes tsconfig.touched.json into a pristine tree and its transformed twin: only touched files plus type libraries, no module following. Usage: mk-tsconfig.py <base> <changed>"""
import json, os, re, subprocess, sys
base, changed = sys.argv[1], sys.argv[2]
diff = subprocess.run(["diff", "-rq", os.path.join(base, "🧰️framework"), os.path.join(changed, "🧰️framework")], capture_output=True, text=True).stdout.splitlines()
touched = []
for line in diff:
    m = re.match(r"Files (.+) and (.+) differ$", line)
    if m: touched.append(os.path.relpath(m.group(2), changed))
ts = sorted(t for t in touched if t.endswith((".ts", ".tsx", ".mts")))
nm = "/Users/ueli/Documents/semio/node_modules"
dts = []
for pkg in ["@types/node", "@types/bun"]:
    r = os.path.realpath(os.path.join(nm, pkg))
    for dp, dn, fn in os.walk(r):
        dts += [os.path.join(dp, f) for f in fn if f.endswith(".d.ts")]
    if pkg == "@types/bun":
        bt = os.path.join(os.path.dirname(os.path.dirname(r)), "bun-types")
        for dp, dn, fn in os.walk(bt):
            if re.search(r"/ts\d", dp + "/"): continue
            dts += [os.path.join(dp, f) for f in fn if f.endswith(".d.ts")]
runtime = []
for dp, dn, fn in os.walk(os.path.join(changed, "🧰️framework/🔨️modules/⚙️runtime")):
    if "🧪️tests" in dp: continue
    runtime += [os.path.relpath(os.path.join(dp, f), changed) for f in fn if f.endswith((".ts", ".json"))]
options = {"target": "ESNext", "module": "ESNext", "moduleResolution": "bundler", "allowImportingTsExtensions": True, "strict": True, "skipLibCheck": True, "noEmit": True, "resolveJsonModule": True, "esModuleInterop": True, "types": [], "noResolve": True, "lib": ["ESNext", "DOM", "DOM.Iterable"]}
for tree, extra in ((base, []), (changed, runtime)):
    files = [f for f in ts + extra if os.path.exists(os.path.join(tree, f))] + dts
    json.dump({"compilerOptions": options, "files": files}, open(os.path.join(tree, "tsconfig.touched.json"), "w", encoding="utf8"), ensure_ascii=False)
    print(tree, len(files))
