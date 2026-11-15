#!/bin/zsh
# usage: suite.sh <overlay name> <log file>; runs the touched-module test files on an overlay and prints "<file> pass fail" per file
HERE=${0:A:h};S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R5c
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";I="$L/🏗️build/🕸️graph/📥️inference"
for t in "$I/🧪️tests" "$I/🧪️tests/🧷️closure-facts" "$I/🧪️tests/🔗️late-closures" "$I/🧪️tests/✂️derived-walks" "$I/🧪️tests/🪆️owner-child-closure" "$I/🧪️tests/🧩️group-merge" "$I/🧪️tests/🔌️providers" "$I/🧪️tests/🧮️covered-inputs" "$I/🧪️tests/🗜️interned-closures" "$I/🧪️tests/🪢️walk-questions" "$I/🧪️tests/🧰️shared-prime" "$I/🧪️tests/🎯️target-fanout" "$I/🧪️tests/🔖️output-stamps" "$I/🧭️script-inputs/🧪️tests" "$I/🧭️script-inputs/🔀️router/🧪️tests" "$I/🧭️script-inputs/👓️walk-views/🧪️tests" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests" "$I/🧱️project-assembly/🧪️tests" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs" "$L/🏗️build/🧪️tests/🕸️graph/🛂️admission" "$L/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests"; do
  out=$(R5_OVERLAY=$S/$1 $HERE/../R5/lab/run-overlay.sh "$t" "🟦️.ts" 2>&1)
  echo "${t#$L/} :: $(echo "$out" | grep -E '^\s*[0-9]+ (pass|fail)' | tr -s ' \n' ' ') :: $(echo "$out" | grep -E '^\(fail\)' | head -3 | cut -c1-120 | tr '\n' ';')" >> "$2"
done
echo done >> "$2"
