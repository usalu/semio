#!/bin/sh
# print-check-tsc.sh : the six tsc --noEmit invocations of `print:check` (CheckScript = typecheck of the package source and its five suites), exactly the flags of artifact:typescript-check
cd /Users/ueli/Documents/semio || exit 1
P="🧰️framework/🛍️products/📓️print"
FLAGS="--noEmit --incremental false --allowImportingTsExtensions --module ESNext --moduleResolution Bundler --resolveJsonModule --allowSyntheticDefaultImports --strict --skipLibCheck --target ES2022 --lib DOM,DOM.AsyncIterable,DOM.Iterable,ESNext"
BUN="${SEMIO_RO_BUN:-bun}"
rc=0
for e in "$P/🟦️.ts" "$P/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts" "$P/🧪️tests/🧬️chart-mutations/🟦️.ts" "$P/🧪️tests/📜️chart-inference-result/🟦️.ts" "$P/🔨️modules/🏠️host/💡️inferences/🧵️worker/🟦️.ts" "$P/🎮️commands/🧪️print-pipeline-verification/🟦️.ts"; do
  echo "== $e"
  "$BUN" node_modules/typescript/bin/tsc "$e" $FLAGS || rc=$?
done
exit $rc
