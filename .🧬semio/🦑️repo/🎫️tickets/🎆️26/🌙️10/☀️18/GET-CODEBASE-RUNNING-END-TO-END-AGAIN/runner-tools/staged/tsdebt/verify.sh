#!/bin/sh
# 🧾 tsdebt verify on a root (mirror or live after apply): `sh verify.sh <root>`. Law 1: framework typecheck has no error in 🏃️process (base: 33 in 10 files). Law 2: graph-phase fixture law + prefetch cases and the statfs walk pass (base: fixture law red, walk times out at 5 s under load).
set -u
ROOT="$1"; OUT="${TMPDIR:-/tmp}/rn-tsdebt.$$"; mkdir -p "$OUT/a"
P="🧰️framework/🔨️modules/🏃️process"
( cd "$ROOT/🧰️framework/📦️packages/🟦️typescript" && node "$ROOT/node_modules/typescript/bin/tsc" -p tsconfig.json --noEmit --incremental false ) > "$OUT/tsc.txt" 2>&1
N=$(grep "error TS" "$OUT/tsc.txt" | grep -c "🏃️process")
echo "law1 process typecheck errors: $N (want 0)"
( cd "$ROOT" && SEMIO_TEST_ARTIFACT_DIR="$OUT/a" bun test "./$P/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️graph-phase/🟦️.ts" "./$P/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs/🧪️tests/🆓️unknown/🟦️.ts" "./$P/🧪️testing/🟨️bun/🚧️operands/🧪️tests/🟦️.ts" "./$P/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🟦️.ts" ) > "$OUT/test.txt" 2>&1
F=$(grep -c "^(fail)" "$OUT/test.txt")
echo "law2 failing tests: $F (want 0)"; grep "^(fail)" "$OUT/test.txt"
rm -rf "$OUT"
[ "$N" = 0 ] && [ "$F" = 0 ]
