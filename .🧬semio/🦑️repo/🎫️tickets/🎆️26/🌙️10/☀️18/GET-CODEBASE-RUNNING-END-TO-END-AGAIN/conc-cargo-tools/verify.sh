#!/bin/sh
# Verifies the staged conc-cargo set on a symlink overlay of the live tree (no write to the repo, one heavy process at a time).
set -e
T=$(cd "$(dirname "$0")" && pwd)
OV=${1:-/private/tmp/conc-cargo/mirror}
ART=${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/conc-cargo/art}
mkdir -p "$ART"
export SEMIO_TEST_ARTIFACT_DIR="$ART"
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
F="🧰️framework/🔨️modules/🏃️process"
python3 "$T/apply.py" check | tail -1
python3 "$T/apply.py" mirror "$OV" | tail -1
cp "$T/wrap.ts" "$OV/"
(cd "$OV" && python3 "$T/tsconfig-overlay.py" && echo "tsc (pre-existing errors only under ♻️reset):" && { node_modules/.bin/tsc -p tsconfig.conc.json 2>&1 | grep -v '^ ' | cut -c1-160 || true; })
echo "== unit cache suite (real cargo, sccache oracle)"
(cd "$OV/$R/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests" && bun test --timeout 170000 ./🟦️.ts 2>&1 | tail -6)
echo "== fetched receipts"
(cd "$OV/$R/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧪️tests" && bun test --timeout 60000 ./🟦️.ts 2>&1 | tail -5)
echo "== admission memo + existing admission suites (1 known failure: files 9/8)"
for T1 in 🧠️memo/🟦️.ts 🟦️.ts 🔁️recheck/🟦️.ts; do (cd "$OV" && bun wrap.ts "./$F/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🧪️tests/$T1" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)' | tr '\n' ' '; echo " <- $T1"); done
echo "== broker replay"
(cd "$OV" && bun wrap.ts "./$F/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/♻️replay/🟦️.ts" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)' | tr '\n' ' '; echo)
echo "== shared writer custody"
(cd "$OV" && bun wrap.ts "./$R/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🤝️shared/🟦️.ts" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)')
