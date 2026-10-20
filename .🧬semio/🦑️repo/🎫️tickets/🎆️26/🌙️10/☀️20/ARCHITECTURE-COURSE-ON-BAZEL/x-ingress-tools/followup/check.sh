#!/bin/sh
# usage: sh check.sh [SCRATCH_DIR]    (default: $TMPDIR/x-ingress-followup)
# 1. read-only --check on the LIVE tree  2. python apply tests (two-phase atomicity)  3. mirror of the live tree + --live  4. explicit-file bun tests (follow-up suite, x-ingress suite, generation suite)  5. tsc over the touched files
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
SCRATCH=${1:-${TMPDIR:-/tmp}/x-ingress-followup}
mkdir -p "$SCRATCH"
python3 "$HERE/apply.py" --check || echo "(rc $? : 1 means edits pending, 2 means a refusal)"
python3 "$HERE/test_apply.py" "$SCRATCH"
python3 "$HERE/apply.py" mirror "$SCRATCH/mirror"
cd "$SCRATCH/mirror"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G="$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
export SEMIO_TEST_ARTIFACT_DIR="$SCRATCH"
for suite in "$G/🧪️tests/🛡️followup/🟦️.ts" "$G/🧪️tests/🚪️ingress/🟦️.ts" "$G/🧪️tests/🟦️.ts"; do
  echo "== $suite"
  bun test "./$suite" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|^error:" || true
done
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$L/🏗️build/🚀️bootstrap/🟦️.ts","$G/🟦️.ts","$G/💾️storage/🟦️.ts","$G/🧪️tests/🛡️followup/🟦️.ts"]}
JSON
node_modules/.bin/tsc -p tsconfig.x.json | grep -F -e "$G" -e "🚀️bootstrap/🟦️.ts" || echo "tsc: no diagnostic in the touched files"
