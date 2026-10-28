#!/bin/sh
# ⏲ innerclock verify on a root (mirror or live after apply): `sh verify.sh <root>`. Law: the new stretch law (Python oracle over 4 levels × 7 contention ratios + the go 45 s/120 s/15 s cases) and the existing levels corpus law pass (base: stretch law 2 red, corpus 8/0); the levels module, both tests and a Go caller type-check (0 errors).
set -u
ROOT="$1"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🎚️levels"; OUT="$ROOT/.rn-innerclock"; mkdir -p "$OUT/tc"
( cd "$ROOT" && bun test "./$L/🧪️tests/⏲️stretch/🟦️.ts" "./$L/🧪️tests/🟦️.ts" ) > "$OUT/law.txt" 2>&1
F=$(grep -c "^(fail)" "$OUT/law.txt"); P=$(sed -n "s/^ *\([0-9][0-9]*\) pass$/\1/p" "$OUT/law.txt"); echo "laws: $P pass, $F fail (want 10/0)"
printf '{"extends":"../../tsconfig.json","compilerOptions":{"noEmit":true,"incremental":false,"allowImportingTsExtensions":true,"resolveJsonModule":true},"include":["../../%s/🟦️.ts","../../%s/🧪️tests/🟦️.ts","../../%s/🧪️tests/⏲️stretch/🟦️.ts","../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🐹️go/📜️script.ts"]}\n' "$L" "$L" "$L" > "$OUT/tc/tsconfig.json"
T=$( cd "$ROOT" && node node_modules/typescript/bin/tsc -p "$OUT/tc/tsconfig.json" 2>&1 | grep -c "error TS" ); echo "typecheck errors: $T (want 0)"
rm -rf "$OUT"
[ "$F" = 0 ] && [ "$P" = 10 ] && [ "$T" = 0 ]
