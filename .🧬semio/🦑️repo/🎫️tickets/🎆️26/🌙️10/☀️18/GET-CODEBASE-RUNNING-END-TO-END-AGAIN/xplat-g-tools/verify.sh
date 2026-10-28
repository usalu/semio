#!/bin/sh
# Single-file graph-free tests of every xplat-g change, run in the tree. Output goes to the ticket generated folder.
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
F=🧰️framework
LB=$F/🛍️products/🦑️repo/🔨️modules/📚️library
OUT="$HERE/../🗑️generated/xplat-g"; mkdir -p "$OUT"
FAIL=0
run() { echo "== $1"; ( cd "$(dirname "$1")" && bun test "./$(basename "$1")" ) > "$OUT/$(echo "$1" | cksum | cut -d' ' -f1).txt" 2>&1 && echo "pass" || { echo "FAIL (see $OUT)"; FAIL=1; }; }
run "$F/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🔒️busy/🟦️.ts"
run "$F/🔨️modules/⚙️runtime/🧪️tests/🛑️termination/🟦️.ts"
run "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🟦️.ts"
run "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🟦️.ts"
run "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts"
run "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts"
run "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts"
run "$LB/🏗️build/🌡️host-capacity/🧪️tests/🪟️partial/🟦️.ts"
run "$LB/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🟦️.ts"
run "$LB/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts"
run "$LB/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🧪️tests/🟦️.ts"
run "$LB/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts"
run "$LB/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts"
run "$LB/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️toolchain-link/🟦️.ts"
run "$LB/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🟦️.ts"
run "$LB/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts"
run "$LB/🎛️commands/📝️commit/🪝️hooks/🧪️tests/🟦️.ts"
echo "== go: cd $F/🛍️products/🦑️repo/🔨️modules/🚀️launcher && the launcher Go test target (TestResidentTrustedDirectoryAndParent)"
exit $FAIL
