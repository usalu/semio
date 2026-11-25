#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
for try in 1 2 3 4 5 6; do
  "$T/carve-test.sh" "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️core/📦️packages/🦀️rust" run-core -p semio-s-artifact-norm-en1993-checks-core
  grep -q "EXIT 143\|dbg has no" "$T/🗑️generated/run-core.log" || break
  sleep 120
done
