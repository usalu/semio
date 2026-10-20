#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/docs-final-tools"
OUT="$C/🗑️generated/docs-final/evidence.txt"
export SEMIO_TEST_ARTIFACT_DIR="$(cd "$C/🗑️generated/docs-final/art" && pwd -P)" SEMIO_WORKSPACE_ROOT="$PWD"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P="🧰️framework/🔨️modules/🏃️process"
D="$L/⚡️caching/🚀️bootstrap/📦️dependencies"
run(){ name="$1"; shift; echo "== $name" >> "$OUT"; perl -e 'alarm 560; exec @ARGV' -- bun test --tsconfig-override "./$T/tsconfig.alias.json" --timeout 240000 "$@" 2>&1 | grep -E "^\(fail\)| pass$| fail$|^error:" | head -12 >> "$OUT"; }
: > "$OUT"
run zero-touch "./$L/🏗️build/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts"
run generation "./$D/🧾️generation/🧪️tests/🟦️.ts"
run deps-paths "./$D/💾️storage/🧪️tests/📏️paths/🟦️.ts"
run deps-fence "./$D/💾️storage/🧪️tests/🚧️fence/🟦️.ts"
run deps-providers "./$D/💾️storage/🧪️tests/🔭️providers/🟦️.ts"
run lease-fairness "./$P/🔒️leases/⚖️fairness/🧪️tests/🟦️.ts"
run registry-consistency "./$L/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts"
echo "== done" >> "$OUT"
