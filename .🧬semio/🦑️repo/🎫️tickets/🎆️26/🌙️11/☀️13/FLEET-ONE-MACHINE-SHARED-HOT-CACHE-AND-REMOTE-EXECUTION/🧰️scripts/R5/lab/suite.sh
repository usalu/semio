#!/bin/zsh
# usage: suite.sh <overlay dir>   runs every affected test file (live location, edited files redirected to the overlay) and prints pass/fail counts
HERE=${0:A:h}; OV=$1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"
run(){ out=$(R5_OVERLAY=$OV SEMIO_TEST_CHECKOUT=/Users/Shared/semio R5_RUN_IN=${3:-/Users/Shared/semio} $HERE/run-overlay.sh "$1" "$2" 2>&1); echo "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: $1/$2" | sed "s|$L/||"; echo "$out" | /usr/bin/grep '^(fail)' | head -5; }
run "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests" '🟦️.ts'
run "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes" '🟦️.ts'
run "$L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests" '🟦️.ts'
run "$B/🕸️graph/📥️inference/🧭️script-inputs/👓️walk-views/🧪️tests" '🟦️.ts'
run "$B/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests" '🟦️.ts'
run "$B/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests" '🟦️.ts'
run "$B/🧪️tests/🕸️graph/🎯️source-demand" '🟦️.ts'
run "$B/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests" '🟦️.ts'
