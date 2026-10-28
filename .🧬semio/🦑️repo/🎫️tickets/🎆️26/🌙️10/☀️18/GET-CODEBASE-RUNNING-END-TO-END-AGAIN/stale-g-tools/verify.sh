#!/bin/sh
# Laws of the stale-g batch (single-file suites). Run AFTER `python3 -I stale-g-tools/stage.py apply` (or inside a mirror that has it). Expected: owner-child 6/0, walk-edits 8/0, output-stamps 4/0, stamps 17/0. Pre-existing reds (not this batch, identical on the base): derived-walks 3/1, workspace-closure 2/4, declared-inputs 3 fail, closure-facts 21/1, course-inputs.
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"; bad=0
for f in "$R/🧪️tests/🪆️owner-child-closure/🟦️.ts" "$R/🧪️tests/🧭️walk-edits/🟦️.ts" "$R/🧪️tests/🔖️output-stamps/🟦️.ts" "$R/🔖️stamps/🧪️tests/🟦️.ts" "$R/🧭️script-inputs/🧪️tests/🟦️.ts"; do
  printf '== %s\n' "$f"; bun test "./$f" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || bad=1
done; exit $bad
