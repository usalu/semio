#!/bin/sh
# 🧹️ laws of batch `rest` (lane lang6): sh verify.sh <root> [before]   (bare bun test; SEMIO_TEST_ARTIFACT_DIR defaults to <root>/.cache/test-sandbox/l6; FAILURE_OUT=<file> receives the sorted pass/fail line per test)
#   after  (default): the batch is applied; the language law holds no finding on the batch's source files; the touched modules' tests run
#   before : the batch is reverted first and applied again afterwards: the law finds the sites on the base bytes (exit 1 means the law is red, as it must be)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; S="$L/⚡️caching/💾️storage/🔒️enforcement"; H="$L/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests"
TESTS="$L/⚖️laws/🧫️test-store-isolation/🧪️tests/🟦️.ts $L/🏗️build/📏️incremental/🏎️bench/🛂️acceptance/🧪️tests/🟦️.ts $H/🟦️.ts $H/🚦️width/🟦️.ts $H/🧷️births/🟦️.ts $H/🧮️resources/🟦️.ts $S/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🔎️scan/🟦️.ts $S/📦️budget/📥️opening/🏪️store/🧪️tests/🆔️identity/🟦️.ts $S/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts $S/🔌️nx/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts $S/🔌️nx/📦️graph/🧪️tests/🤝️yield/🟦️.ts"
ART="${SEMIO_TEST_ARTIFACT_DIR:-$ROOT/.cache/test-sandbox/l6}"; mkdir -p "$ART"
if [ "$MODE" = before ]; then python3 -I "$HERE/stage.py" revert --root "$ROOT" >/dev/null || { echo "revert refused"; exit 2; }; else python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }; fi
(cd "$ROOT" && CP_ROOT="$ROOT" bun "$HERE/../../lawpaths.ts" "$HERE/manifest.json" 2>&1 | grep -v DEBUG); LAW=$?
n=0; cd "$ROOT" && for t in $TESTS; do n=$((n+1)); env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 170000 "./$t" 2>&1 | grep -E '^\((pass|fail)\)|^ *[0-9]+ (pass|fail)$' | sed 's/ \[[0-9.]*ms\]$//' | sed "s|^ *|t$n |"; done | sort > "${FAILURE_OUT:-/dev/stdout}"
if [ "$MODE" = before ]; then python3 -I "$HERE/stage.py" apply --root "$ROOT" >/dev/null; exit 0; fi
exit 0
