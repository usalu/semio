#!/bin/zsh
# [fdleak-g] verifies the guard batch on the tree after `stage.py apply` (census2-g's batch applied first): graph-free single-file suites with their expected pass counts, all run under `nice`, one at a time; exit 1 on any difference.
# usage: verify.sh [--root DIR]   (default: the live checkout)
R="/Users/ueli/Documents/semio"; [ "$1" = "--root" ] && R="$2"
cd "$R" || exit 1
P="🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun"
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/tmp/fdleak-g-art}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
fail=0
run(){ local dir="$1" want="$2"; got=$(cd "$dir" && nice bun test --timeout 280000 ./🟦️.ts 2>&1 | grep -E "^ *[0-9]+ pass$" | tail -1 | tr -d ' '); if [ "$got" = "${want}pass" ]; then echo "ok   $dir ($got)"; else echo "FAIL $dir (want ${want}pass, got ${got:-none})"; fail=1; fi; }
run "$P/🪦️orphans/🧪️tests" 10
run "$P/🚧️operands/🧪️tests" 5
run "$P/🧯️ceiling/🧪️tests" 7
run "🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests" 21
exit $fail
