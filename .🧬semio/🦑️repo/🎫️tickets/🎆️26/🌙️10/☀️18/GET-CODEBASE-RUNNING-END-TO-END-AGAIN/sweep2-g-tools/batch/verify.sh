#!/bin/sh
# sweep2-g verify: run AFTER `python3 apply.py apply`. Single-file tests only, every operand starts with ./ (a bare operand is a filter scan of the checkout).
cd /Users/ueli/Documents/semio || exit 1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
S="$L/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests"
fail=0
run() { (cd "$1" && nice -n 10 bun test "./$2") || fail=1; }
run "$S" "🟦️.ts"
run "$S/🧵️unit-cache-lanes" "🟦️.ts"
run "$L/🏗️build/📥️inputs/🧪️tests/🔑️test-data-reads" "🟦️.ts"
run "$L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests" "🟦️.ts"
run "$L/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests" "🟦️.ts"
run "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests" "🟦️.ts"
exit $fail
