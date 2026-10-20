#!/bin/sh
# usage: verify.sh <overlay> [file ...]   runs each Bazel adapter test file alone (one process at a time) and prints its tally
OV=${1:-/private/tmp/conc-bazel/ov}; shift
cd "$OV" || exit 9
Z="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
FILES=${*:-"💾️storage/🧪️tests/🟦️.ts ⚙️operations/🧪️tests/🟦️.ts 🧪️tests/🔏️implementation/🟦️.ts 🧪️tests/📥️actions/🟦️.ts 🧪️tests/📥️workspace/🟦️.ts 🧪️tests/📥️platform/🟦️.ts 🧪️tests/📥️delegation/🟦️.ts 🧪️tests/📥️sources/🟦️.ts 🧪️tests/📥️fingerprint/🟦️.ts 🧪️tests/📥️outputs/🟦️.ts 🧪️tests/📥️closure/🟦️.ts 🧵️sessions/🧪️tests/🟦️.ts"}
for f in $FILES; do
  out=$(BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 bun test "./$Z/$f" 2>&1)
  echo "== $f: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail|skip)' | tr '\n' ' ')"
  echo "$out" | grep -E "^\(fail\)" | head -8
done
