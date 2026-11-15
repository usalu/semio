#!/bin/zsh
# usage: suite.sh <overlay name> <log file> [test dirs relative to LIB...]; runs the touched-module test files with the overlay and prints "<dir> :: pass fail :: first red lines" per dir
HERE=${0:A:h};S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";I="$L/🏗️build/🕸️graph/📥️inference";SI="$I/🧭️script-inputs"
DIRS=("$SI/🔌️provider-closures/🧪️tests" "$SI/🔌️provider-closures/🧪️tests/🪡️tool-cover" "$SI/🧪️tests" "$SI/🔀️router/🧪️tests" "$SI/👓️walk-views/🧪️tests" "$I/🧪️tests/🧷️closure-facts" "$I/🧪️tests/🔖️output-stamps" "$I/🧪️tests/🔗️late-closures" "$I/🧱️project-assembly/🧪️tests" "$L/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs" "$L/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests" "$I/🔖️stamps/🧪️tests")
[[ -n "$3" ]] && DIRS=("${@:3}")
for t in $DIRS; do
  out=$($HERE/runlocal.sh "$1" "$t" 2>&1)
  echo "${t#$L/} :: $(echo "$out" | grep -E '^\s*[0-9]+ (pass|fail)' | tr -s ' \n' ' ') :: $(echo "$out" | grep -E '^\(fail\)' | head -3 | cut -c1-140 | tr '\n' ';')" >> "$2"
done
echo done >> "$2"
