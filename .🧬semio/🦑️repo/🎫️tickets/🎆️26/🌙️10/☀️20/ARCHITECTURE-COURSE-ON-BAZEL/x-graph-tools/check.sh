#!/bin/sh
# usage: sh check.sh [SCRATCH_DIR]      (default: $TMPDIR/x-graph)
# 1. read-only --check on the LIVE tree  2. two-phase apply tests  3. fixture reproducible from the generator  4. mirror of the live tree + apply
# 5. oracle verifies the frozen answers (hashlib, jsonschema)  6. explicit-file bun suite (Ajv, graphlib, real observation module, fresh processes)
# 7. red/green mutation proof  8. tsc over the touched files
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
SCRATCH=${1:-${TMPDIR:-/tmp}/x-graph}
mkdir -p "$SCRATCH"
python3 "$HERE/apply.py" --check || echo "(rc $?: 1 means edits pending, 2 means a refusal)"
python3 "$HERE/test_apply.py" "$SCRATCH"
python3 "$HERE/gen_fixture.py" | cmp - "$HERE/staged/closure-cache.fixture.json" && echo "fixture is reproducible from gen_fixture.py"
python3 "$HERE/apply.py" mirror "$SCRATCH/mirror"
cd "$SCRATCH/mirror"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T="$B/🕸️graph/🚪️admission/🧪️tests/🗃️closure-cache"
python3 "$T/🐍️oracle.py" "$T/🔣️.json" "$B/🧬️schema/🚪️admission.json" --verify
export SEMIO_TEST_ARTIFACT_DIR="$SCRATCH"
bun test "./$T/🟦️.ts" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|^error:" || true
bun test "./$B/🕸️graph/🚪️admission/🧪️tests/🟦️.ts" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|^error:" || true
python3 "$HERE/mutation-check.py" "$SCRATCH/mirror"
cat > tsconfig.x.json <<JSON
{"extends":"./tsconfig.json","compilerOptions":{"noEmit":true,"types":["bun"]},"include":[],"files":["$B/🕸️graph/🟦️.ts","$B/🕸️graph/🚪️admission/🟦️.ts","$T/🟦️.ts","$T/🏃️step/🟦️.ts"]}
JSON
node_modules/.bin/tsc -p tsconfig.x.json | grep -F -e "$B/🕸️graph/🟦️.ts" -e "$B/🕸️graph/🚪️admission/🟦️.ts" -e "closure-cache" || echo "tsc: no diagnostic in the touched files"
