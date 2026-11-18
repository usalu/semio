#!/bin/zsh
LABEL=$1
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R100
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
B=$G/🏗️build/🕸️graph/📥️inference
H=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
SI=$B/🧭️script-inputs
PC=$SI/🔌️provider-closures/🧪️tests
: > $SC/$LABEL.txt
LIST=("$PC/🛑️tool-boundary/🟦️.ts" "$PC/🪡️tool-cover/🟦️.ts" "$PC/🧮️blast-radius/🟦️.ts" "$PC/🟦️.ts" "$PC/🛃️preflight/🟦️.ts" "$SI/🪆️heavy-questions/🧪️tests/🟦️.ts" "$SI/🧪️tests/🟦️.ts" "$SI/🧪️tests/🧽️walk-maps/🟦️.ts" "$SI/🧪️tests/🪁️program-reads/🟦️.ts" "$SI/🧪️tests/🫧️take-release/🟦️.ts" "$SI/🧪️tests/closure-answer-refresh.test.ts" "$SI/🧪️tests/walk-members-merge.test.ts" "$SI/🧪️tests/without-scheduling.test.ts" "$SI/🧪️tests/producer-path.test.ts" "$G/🏗️build/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts" "$B/📦️project-nodes/🧪️tests/📥️take/🟦️.ts")
[ "$2" = fast ] && LIST=("$PC/🛑️tool-boundary/🟦️.ts" "$PC/🪡️tool-cover/🟦️.ts" "$PC/🛃️preflight/🟦️.ts" "$SI/🪆️heavy-questions/🧪️tests/🟦️.ts")
for f in $LIST; do
  [ -f "$f" ] || { echo "MISSING ${f#$G/}" >> $SC/$LABEL.txt; continue; }
  t0=$(date +%s)
  out=$($BUN $H --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
  pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ')
  fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ')
  echo "${f#$G/} :${pass}${fail} ($(( $(date +%s) - t0 ))s)" >> $SC/$LABEL.txt
done
echo done >> $SC/$LABEL.txt
