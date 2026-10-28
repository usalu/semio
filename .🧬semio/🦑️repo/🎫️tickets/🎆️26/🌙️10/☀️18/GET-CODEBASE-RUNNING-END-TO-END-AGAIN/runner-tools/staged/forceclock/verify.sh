#!/bin/sh
# 🧲 forceclock verify on a root (mirror holding the generated dirs, or live after apply): `sh verify.sh <root>`. Law: the whole owned Diagram component suite passes (55/55 on rnfw). Red-on-base evidence (result-runner.md): a slow clock (3 ms per performance.now() call) reproduces the live failure on base; the fix pins the test to the deterministic budget clock.
set -u
ROOT="$1"; R="🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"; OUT="$ROOT/.rn-forceclock"; mkdir -p "$OUT/c" "$OUT/v"
POL="{\"version\":1,\"cwd\":\"$ROOT/$R\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$OUT/v\",\"coverageDirectory\":\"$OUT/c\",\"budgetMs\":600000}"
( cd "$ROOT/$R" && SEMIO_VITEST_POLICY="$POL" node "$ROOT/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native "../../../../🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx" ) > "$OUT/out.txt" 2>&1
grep -E "Tests  |AssertionError" "$OUT/out.txt" | head -3
F=$(grep -cE "Tests .*failed" "$OUT/out.txt"); P=$(grep -cE "Tests .*passed" "$OUT/out.txt"); rm -rf "$OUT"
[ "$F" = 0 ] && [ "$P" = 1 ]
