#!/bin/sh
# usage: sh check.sh [SCRATCH_DIR]   (default: $TMPDIR/x-ingress)
# 1. dry-run of the anchored apply on the LIVE tree (read only)  2. mirror + apply  3. explicit-file bun test in the mirror  4. tsc over the four touched files
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
SCRATCH=${1:-${TMPDIR:-/tmp}/x-ingress}
mkdir -p "$SCRATCH"
python3 "$HERE/apply.py" check || echo "(pending operations above are expected before apply)"
python3 "$HERE/mirror-test.py" "$SCRATCH"
cd "$SCRATCH/mirror"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G="$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$L/🏗️build/🚀️bootstrap/🟦️.ts","$G/🟦️.ts","$G/💾️storage/🟦️.ts","$G/🧪️tests/🚪️ingress/🟦️.ts"]}
JSON
node_modules/.bin/tsc -p tsconfig.x.json | grep -F -e "$G" -e "🚀️bootstrap/🟦️.ts" || echo "tsc: no diagnostic in the touched files"
