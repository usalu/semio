#!/bin/sh
# Type-checks the STAGED modified live files (adapter, operations, verify, neutral script) against the live closure through rootDirs; prints errors by file so pre-existing ones are visible.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT=/Users/ueli/Documents/semio
OV=/private/tmp/parity/ov
python3 "$HERE/overlay.py" > /dev/null
python3 - <<'PY'
import json, os, shutil
HERE = os.environ.get("HERE") or "."
PY
python3 - "$HERE" "$OV" <<'PY'
import json, os, shutil, sys
here, ov = sys.argv[1], sys.argv[2]
layout = json.load(open(here + "/layout.json", encoding="utf-8"))
for flat, rel in layout["modified"].items():
    if flat.endswith(".ts"):
        dst = os.path.join(ov, rel); os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copy2(os.path.join(here, "staged/mod", flat), dst)
PY
P="$OV/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
rm -f "$P/🧭️routing/🟦️.ts" "$P/🎛️execution/🟦️.ts"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
cat > "$OV/tsconfig.json" <<JSON
{"compilerOptions":{"target":"ESNext","module":"ESNext","lib":["DOM","DOM.Iterable","ESNext"],"moduleResolution":"bundler","strict":true,"esModuleInterop":true,"isolatedModules":true,"resolveJsonModule":true,"allowImportingTsExtensions":true,"skipLibCheck":true,"noEmit":true,"typeRoots":["$ROOT/node_modules/@types"],"types":["bun","node"],"rootDirs":["$OV","$ROOT"]},
"files":["$B/🔧️tools/🏗️bazel/📜️script.ts","$B/🔧️tools/🏗️bazel/⚙️operations/🟦️.ts","$B/📜️script.ts","$L/🎛️commands/✅️verify/🟦️.ts","$L/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts"]}
JSON
cd "$OV"
perl -e 'alarm 560; exec @ARGV' -- bun "$ROOT/node_modules/typescript/bin/tsc" -p tsconfig.json "$@" || true
