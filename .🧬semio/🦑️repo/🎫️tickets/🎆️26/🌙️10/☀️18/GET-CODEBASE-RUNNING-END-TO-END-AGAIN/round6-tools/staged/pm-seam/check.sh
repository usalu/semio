#!/bin/sh
# usage: sh check.sh [MIRROR_DIR]   1 anchors on the live tree (read only)  2 mirror with lock-sync closure + this set  3 tsc over the touched and new files
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/pm-seam/m}
python3 "$HERE/apply.py" check >/dev/null || echo "(pending operations are expected before apply)"
[ -d "$M/node_modules" ] || python3 "$HERE/apply.py" mirror "$M" --lock-sync --apply | tail -2
cd "$M"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"
M2="$D/🔌️managers"
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$M2/🟦️.ts","$M2/📖️formats/🟦️.ts","$M2/📚️catalog/🟦️.ts","$M2/🧪️tests/🟦️.ts","$D/💾️storage/🟦️.ts","$D/🧾️incremental/💾️storage/🟦️.ts","$L/🏗️build/🚀️bootstrap/🟦️.ts","$L/🏗️build/🕸️graph/🟦️.ts","$L/🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts"]}
JSON
nice -n 10 node_modules/.bin/tsc -p tsconfig.x.json | command grep -F -e "$M2" -e "$D/💾️storage/🟦️.ts" -e "$D/🧾️incremental/💾️storage/🟦️.ts" -e "🏗️build/🚀️bootstrap/🟦️.ts" -e "🏗️build/🕸️graph/🟦️.ts" -e "👁️observation/🧪️tests/⚡️incremental/🟦️.ts" || echo "tsc: no diagnostic in the touched files"
