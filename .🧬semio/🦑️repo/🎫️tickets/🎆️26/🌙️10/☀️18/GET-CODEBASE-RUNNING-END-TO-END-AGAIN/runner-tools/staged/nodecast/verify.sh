#!/bin/sh
# 🟢 nodecast verify on a root (mirror or live after apply): `sh verify.sh <root>`. Law 1: Node 24 strip-types loads the closures module (base: ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX on the `<[string,string][]>` cast). Law 2: the os-dev contribution suite (Bun, Node and Ajv) passes 4/4 (base 3/4 with the same SyntaxError, as in the official rn-os run).
set -u
ROOT="$1"; F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts"; R="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"; OUT="$ROOT/.rn-nodecast"; mkdir -p "$OUT/v" "$OUT/c" "$OUT/a"
A=$( cd "$ROOT" && node --experimental-strip-types --no-warnings --input-type=module -e "await import('file://$ROOT/$F');console.log('loaded')" 2>&1 | grep -c "^loaded$" ); echo "law1 node strip-types load: $A (want 1)"
POL="{\"version\":1,\"cwd\":\"$ROOT/$R\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$OUT/v\",\"coverageDirectory\":\"$OUT/c\",\"budgetMs\":600000}"
( cd "$ROOT/$R" && SEMIO_VITEST_POLICY="$POL" SEMIO_TEST_ARTIFACT_DIR="$OUT/a" node "$ROOT/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native ../../🧪️tests/🧩️contribution/🟦️.ts ) > "$OUT/t.txt" 2>&1
grep -E "Tests  " "$OUT/t.txt"; B=$(grep -cE "Tests  4 passed \(4\)" "$OUT/t.txt"); rm -rf "$OUT"
[ "$A" = 1 ] && [ "$B" = 1 ]
