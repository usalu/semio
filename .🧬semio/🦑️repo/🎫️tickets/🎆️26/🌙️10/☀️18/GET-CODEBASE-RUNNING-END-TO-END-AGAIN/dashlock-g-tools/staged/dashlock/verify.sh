#!/bin/sh
# verify.sh [root]: the laws of the dashlock-g batch, single-file runs, graph-free (run after `python3 -I dashlock-g-tools/stage.py apply`): digest publication/retention laws against real leases and live holder processes, the dashboard retention table, and a strict type check of the three changed modules.
cd "${1:-/Users/ueli/Documents/semio}" || exit 1
F="🧰️framework"; P="$F/🔨️modules/🏃️process/📦️artifacts/📤️publication"; I="$F/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation"
set -e
for t in "$P/🧊️digest/🧪️tests/🟦️.ts" "$I/🧪️tests/🟦️.ts"; do
  echo "== $t"; nice -n 10 bun test --timeout 120000 "./$t" 2>&1 | tail -6
done
echo "== tsc"
nice -n 10 bun node_modules/typescript/bin/tsc --noEmit --strict --skipLibCheck --allowImportingTsExtensions --resolveJsonModule --esModuleInterop --target ESNext --module ESNext --moduleResolution bundler --types bun "$P/🟦️.ts" "$P/🧊️digest/🧪️tests/🟦️.ts" "$I/🟦️.ts" "$I/🧪️tests/🟦️.ts"
echo "tsc clean"
echo "== official route (gate-g, one run): the transpiled pin law needs an issued invocation; run the owning suite of $F/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📌️executable/🧪️tests/🟦️.ts (testCoreExecutablePinControlV1) and the dashboard :test target."
