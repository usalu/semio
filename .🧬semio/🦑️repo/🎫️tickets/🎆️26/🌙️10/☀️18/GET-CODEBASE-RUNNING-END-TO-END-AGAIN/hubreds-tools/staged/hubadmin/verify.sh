#!/bin/sh
# 🛡️ hubadmin verify on a root (mirror, or live after apply): `sh verify.sh <root>`. Law: the admin vitest files command routing and the new build-config law that loads the owned Vite configuration through Vite's runner and native config loaders) passes. Red on base: `ReferenceError: require is not defined` from the CommonJS payload file reached through the playground barrel (os-hub-admin:build, rn-hub).
ROOT="$1"; H=$(cd "$(dirname "$0")" && pwd); P="🌎️hub/🔨️modules/🛡️admin"; OUT="$ROOT/.hubreds-admin"; mkdir -p "$OUT/c" "$OUT/v"
python3 -I "$H/stage.py" check --root "$ROOT" | grep -v '^applied' && exit 1
POL="{\"version\":1,\"cwd\":\"$ROOT/$P/📦️packages/🟦️typescript\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$OUT/v\",\"coverageDirectory\":\"$OUT/c\",\"budgetMs\":600000}"
( cd "$ROOT/$P/📦️packages/🟦️typescript" && SEMIO_VITEST_POLICY="$POL" node "$ROOT/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native ../../🧪️tests/🧪️build-config/🟦️.ts ../../🧪️tests/🧪️command-routing/🟦️.ts ) > "$OUT/out.txt" 2>&1
grep -E "Tests  |Test Files|require is not defined|FAIL" "$OUT/out.txt" | head -6
F=$(grep -cE "Tests .*failed" "$OUT/out.txt"); P=$(grep -cE "Tests .*passed" "$OUT/out.txt"); rm -rf "$OUT"
[ "$F" = 0 ] && [ "$P" = 1 ]
