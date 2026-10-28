#!/bin/sh
# 🎟 devslot verify on a root (mirror, or live after apply): `sh verify.sh <root>`. Law: the new service-capacity law passes (base: source law red, the listener asks for the counted share); the compile-units suite passes through the runner envelope (`C/red-g-tools/mrun.sh <mirror> chain`, 14/0 on rnfw) and bare only fails its 4 envelope-only tests; the 3 TS files type-check (0 errors).
set -u
ROOT="$1"; V="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite"; U="🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units"
OUT="$ROOT/.rn-devslot"; mkdir -p "$OUT/a" "$OUT/tc"
( cd "$ROOT" && SEMIO_TEST_ARTIFACT_DIR="$OUT/a" bun test "./$V/🧪️tests/🎟️service-capacity/🟦️.ts" ) > "$OUT/law.txt" 2>&1
L=$(grep -c "^(fail)" "$OUT/law.txt"); P=$(sed -n "s/^ *\([0-9][0-9]*\) pass$/\1/p" "$OUT/law.txt"); echo "law service-capacity: $P pass, $L fail (want 2/0)"
printf '{"extends":"../../tsconfig.json","compilerOptions":{"noEmit":true,"incremental":false,"allowImportingTsExtensions":true},"include":["../../%s/🎟️capacity/🟦️.ts","../../%s/🧪️tests/🎟️service-capacity/🟦️.ts","../../%s/🟦️.ts"]}\n' "$V" "$V" "$V" > "$OUT/tc/tsconfig.json"
T=$( cd "$ROOT" && node node_modules/typescript/bin/tsc -p "$OUT/tc/tsconfig.json" 2>&1 | grep -c "error TS" ); echo "typecheck errors: $T (want 0)"
rm -rf "$OUT"
[ "$L" = 0 ] && [ "$P" = 2 ] && [ "$T" = 0 ]
