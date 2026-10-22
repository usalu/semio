#!/bin/sh
# usage: sh check.sh [MIRROR_DIR]   anchors on the live tree, mirror with the set applied, tsc over the touched files
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/si/m3}
python3 "$HERE/apply.py" check >/dev/null || echo "(pending operations are expected before apply)"
python3 "$HERE/apply.py" mirror "$M" --apply | tail -1
cd "$M"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$D/🪞️adopt/🟦️.ts","$D/🪞️adopt/💾️storage/🟦️.ts","$D/🪞️adopt/🧪️tests/🟦️.ts","$D/💾️storage/🟦️.ts"]}
JSON
node_modules/.bin/tsc -p tsconfig.x.json | command grep -F -e "$D/🪞️adopt" -e "$D/💾️storage/🟦️.ts" || echo "tsc: no diagnostic in the touched files"
