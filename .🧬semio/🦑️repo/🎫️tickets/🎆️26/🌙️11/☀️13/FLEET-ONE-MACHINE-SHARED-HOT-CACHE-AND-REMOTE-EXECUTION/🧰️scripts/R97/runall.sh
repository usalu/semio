#!/bin/zsh
LABEL=$1
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R97
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
BD=$G/🏗️build
B=$BD/🕸️graph/📥️inference
N=$B/📦️project-nodes
LIST=("$BD"/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts "$BD"/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🌊️streamed-intake/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🧮️admission/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🧯️cap/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🛑️hold/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🔱️warm-jobs/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🟦️.ts "$N"/🏊️project-farm/🧪️tests/🏘️hub/🟦️.ts "$N"/🌐️owner-pool/🧪️tests/🟦️.ts "$B"/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts "$B"/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts)
: > $SC/$LABEL.txt
for f in $LIST; do
  [ -f "$f" ] || { echo "MISSING ${f#$BD/}" >> $SC/$LABEL.txt; continue; }
  for attempt in 1 2 3; do
    out=$($BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
    pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ')
    fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ')
    echo "$fail" | grep -qE "[1-9][0-9]* fail" || break
  done
  echo "${f#$BD/} :${pass}${fail} (attempt $attempt)" >> $SC/$LABEL.txt
done
echo done >> $SC/$LABEL.txt
