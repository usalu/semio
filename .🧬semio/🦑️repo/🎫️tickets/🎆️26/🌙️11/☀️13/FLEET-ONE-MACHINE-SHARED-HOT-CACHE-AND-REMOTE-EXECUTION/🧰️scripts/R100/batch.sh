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
LIST=("$PC/🛑️tool-boundary/🟦️.ts" "$PC/🪡️tool-cover/🟦️.ts" "$PC/🧮️blast-radius/🟦️.ts" "$PC/🛃️preflight/🟦️.ts" "$SI/🪆️heavy-questions/🧪️tests/🟦️.ts" "$SI/🧪️tests/🫧️take-release/🟦️.ts" "$SI/🧪️tests/🪁️program-reads/🟦️.ts" "$SI/🧪️tests/🧽️walk-maps/🟦️.ts" "$SI/🧪️tests/closure-answer-refresh.test.ts" "$SI/🧪️tests/walk-members-merge.test.ts" "$SI/🧪️tests/without-scheduling.test.ts" "$SI/🧪️tests/unresolved-workspace-import.test.ts" "$SI/🧪️tests/provider-project-field.test.ts" "$G/🏗️build/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts" "$B/📦️project-nodes/🧪️tests/📥️take/🟦️.ts" "$B/📦️project-nodes/📥️take/🧪️tests/🟦️.ts" "$B/🧪️tests/🧷️closure-facts/🟦️.ts" "$B/🧪️tests/🪢️walk-questions/🟦️.ts" "$B/🧪️tests/🧭️walk-edits/🟦️.ts" "$B/🧪️tests/✂️derived-walks/🟦️.ts" "$B/🧪️tests/🔗️late-closures/🟦️.ts" "$B/🧪️tests/🔗️workspace-closure/🟦️.ts" "$B/🧪️tests/🧮️covered-inputs/🟦️.ts" "$B/🧪️tests/🪤️take-no-derive/🟦️.ts" "$B/🧪️tests/🧰️shared-prime/🟦️.ts" "$B/🧪️tests/🧩️group-merge/🟦️.ts" "$B/🧪️tests/🧮️module-memo/🟦️.ts" "$B/🧪️tests/🧮️pass-verdicts/🟦️.ts" "$B/🧪️tests/🪆️owner-child-closure/🟦️.ts" "$B/🧪️tests/🪪️layer-ids/🟦️.ts" "$B/🧪️tests/♻️owner-reuse/🟦️.ts" "$B/🧪️tests/🦀️compiled-closure/🟦️.ts" "$B/🧪️tests/🧺️bundle-rows/🟦️.ts")
mkdir -p $SC/out/$LABEL
i=0
for f in $LIST; do
  i=$((i+1))
  [ -f "$f" ] || { echo "$i MISSING ${f#$G/}" > $SC/out/$LABEL/$i.txt; continue; }
  ( t0=$(date +%s); out=$($BUN $H --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' '); fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' '); echo "$i ${f#$G/} :${pass}${fail} ($(( $(date +%s) - t0 ))s)" > $SC/out/$LABEL/$i.txt; echo "$out" > $SC/out/$LABEL/$i.log ) &
done
wait
cat $SC/out/$LABEL/*.txt | sort -n > $SC/$LABEL.txt
echo done >> $SC/$LABEL.txt
