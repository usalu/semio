#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/docs-final-tools"
OUT="$C/🗑️generated/docs-final/evidence2.txt"
export SEMIO_TEST_ARTIFACT_DIR="$(cd "$C/🗑️generated/docs-final/art" && pwd -P)" SEMIO_WORKSPACE_ROOT="$PWD" SEMIO_TEST_BAZEL_BINARY="$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build/🔧️tools/🏗️bazel"
run(){ name="$1"; shift; echo "== $name" >> "$OUT"; perl -e 'alarm 590; exec @ARGV' -- bun "$C/bzl-act-tools/wrap.ts" --tsconfig-override "./$T/tsconfig.alias.json" --timeout 300000 "$@" 2>&1 | grep -E "^\(fail\)| pass$| fail$|^error:" | head -12 >> "$OUT"; }
: > "$OUT"
run workspace "./$B/🧪️tests/📥️workspace/🟦️.ts"
run sources "./$B/🧪️tests/📥️sources/🟦️.ts"
run actions "./$B/🧪️tests/📥️actions/🟦️.ts"
run fingerprint "./$B/🧪️tests/📥️fingerprint/🟦️.ts"
run closure "./$B/🧪️tests/📥️closure/🟦️.ts"
run store-receipt "./$L/🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts"
echo "== done" >> "$OUT"
