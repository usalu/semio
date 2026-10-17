"""📏️ Runs tsc --noEmit over a list of root files and writes the diagnostics, one per line, to the generated folder.

usage: tsc-run.py <phase> <roots.txt>
"""
import json, os, subprocess, sys
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
phase, roots = sys.argv[1], sys.argv[2]
files = [l.strip() for l in open(roots, encoding="utf-8") if l.strip()]
cfg = {
    "extends": R + "/tsconfig.json",
    "compilerOptions": {
        "allowImportingTsExtensions": True, "noEmit": True, "skipLibCheck": True, "incremental": False, "types": ["bun", "node"],
        "paths": {
            "dom-accessibility-api": [R + "/node_modules/dom-accessibility-api/dist/index.d.ts"],
            "fast-json-stable-stringify": [R + "/🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts"],
            "lines-and-columns": [R + "/node_modules/lines-and-columns/build/index.d.ts"],
        },
    },
    "include": [],
    "files": [f if f.startswith("/") else R + "/" + f for f in files if os.path.exists(f if f.startswith("/") else R + "/" + f)],
}
path = G + "/c4p21-tsc-%s.json" % phase
if os.environ.get("C4P21_UNUSED"):
    cfg["compilerOptions"]["noUnusedLocals"] = True
json.dump(cfg, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
out = subprocess.run(["node", R + "/node_modules/typescript/bin/tsc", "-p", path, "--noEmit", "--pretty", "false"], capture_output=True, text=True, cwd=R)
open(G + "/c4p21-tsc-%s.txt" % phase, "w", encoding="utf-8").write(out.stdout + out.stderr)
print("exit", out.returncode, "files", len(cfg["files"]), "lines", len((out.stdout + out.stderr).splitlines()))
