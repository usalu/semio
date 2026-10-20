#!/bin/sh
# usage: sh check.sh [MIRROR_DIR]   (default /private/tmp/conc-deps/m)
# 1 anchors on the live tree (read only)  2 fresh mirror with the set applied  3 tsc over the touched files  (tests: sh test.sh)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/conc-deps/m}
python3 "$HERE/apply.py" check >/dev/null || echo "(pending operations are expected before apply)"
python3 "$HERE/apply.py" mirror "$M" --apply --group ${GROUP:-3} | tail -1
cd "$M"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$D/🚦️concurrent/🟦️.ts","$D/🚦️concurrent/💾️storage/🟦️.ts","$D/💾️storage/🟦️.ts","$L/🏗️build/🚀️bootstrap/🟦️.ts","$D/🚦️concurrent/🧪️tests/🟦️.ts","$D/🚦️concurrent/🧪️tests/📖️reader.ts"]}
JSON
node_modules/.bin/tsc -p tsconfig.x.json | command grep -F -e "$D/🚦️concurrent" -e "$D/💾️storage/🟦️.ts" -e "🏗️build/🚀️bootstrap/🟦️.ts" || echo "tsc: no diagnostic in the touched files"
