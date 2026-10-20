#!/bin/sh
# Type-checks the staged parity files against the REAL process-control modules: a tsc project whose rootDirs merge the overlay with the repo (read-only), the stub modules removed.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT=/Users/ueli/Documents/semio
OV=/private/tmp/parity/ov
python3 "$HERE/overlay.py" > /dev/null
P="$ROOT/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
O="$OV/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
rm -f "$O/🧭️routing/🟦️.ts" "$O/🎛️execution/🟦️.ts"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cat > "$OV/tsconfig.json" <<JSON
{"compilerOptions":{"target":"ESNext","module":"ESNext","lib":["DOM","DOM.Iterable","ESNext"],"moduleResolution":"bundler","strict":true,"esModuleInterop":true,"isolatedModules":true,"resolveJsonModule":true,"allowImportingTsExtensions":true,"skipLibCheck":true,"noEmit":true,"typeRoots":["$ROOT/node_modules/@types"],"types":["bun","node"],"rootDirs":["$OV","$ROOT"]},
"files":["$L/🪞️parity/🟦️.ts","$L/🪞️parity/📥️evidence/🟦️.ts","$L/🪞️parity/🏃️collect/🟦️.ts","$L/🪞️parity/📝️report/🟦️.ts","$L/🪞️parity/⌨️command/🟦️.ts","$L/🪞️parity/🧪️tests/🟦️.ts","$L/🔧️tools/🏗️bazel/👁️evidence/🟦️.ts","$L/🔧️tools/🏗️bazel/👁️evidence/🧪️tests/🟦️.ts"]}
JSON
cd "$OV"
exec perl -e 'alarm 480; exec @ARGV' -- bun "$ROOT/node_modules/typescript/bin/tsc" -p tsconfig.json "$@"
