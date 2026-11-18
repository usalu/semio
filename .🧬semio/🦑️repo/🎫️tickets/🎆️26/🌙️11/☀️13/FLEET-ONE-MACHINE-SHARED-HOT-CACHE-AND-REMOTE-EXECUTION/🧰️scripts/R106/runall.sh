#!/bin/zsh
LABEL=$1
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R106
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
B=$G/🏗️build
I=$B/🕸️graph/📥️inference
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
N=$I/📦️project-nodes
GS=$B/🪪️graph-state
: > $SC/$LABEL.txt
LIST=("$N"/📥️take/🧪️tests/🟦️.ts "$N"/📥️take/🧪️tests/🚀️release/🟦️.ts "$N"/📥️take/🧪️tests/🤝️two-machines/🟦️.ts "$N"/📥️take/🧪️tests/🚦️no-progress/🟦️.ts "$GS"/🧪️tests/⏳️lease-wait/🟦️.ts "$GS"/🧪️tests/🟦️.ts "$GS"/📦️project-records/🧪️tests/🟦️.ts "$GS"/📦️project-records/🧪️tests/📮️retry/🟦️.ts "$N"/🧪️tests/📥️take/🟦️.ts "$N"/🧪️tests/📈️inferred/🟦️.ts "$N"/🧪️tests/🚪️away/🟦️.ts "$N"/🧪️tests/🐌️slow-last/🟦️.ts "$N"/🧪️tests/🛑️too-heavy/🟦️.ts "$N"/🧪️tests/🛟️stopped-wave/🟦️.ts "$N"/🧪️tests/🧵️ready-context/🟦️.ts "$N"/🧪️tests/🛣️lanes/🟦️.ts "$N"/🧪️tests/🪂️set-aside/🟦️.ts "$N"/🌐️owner-pool/🧪️tests/🟦️.ts "$B"/👁️observation/🧪️tests/📦️projects/🟦️.ts "$B"/👁️observation/🧪️tests/🔎️lookup-holds/🟦️.ts "$I"/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts "$I"/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts)
if [ "$2" = fast ]; then LIST=("$N"/📥️take/🧪️tests/🚦️no-progress/🟦️.ts "$GS"/🧪️tests/⏳️lease-wait/🟦️.ts "$GS"/🧪️tests/🟦️.ts "$GS"/📦️project-records/🧪️tests/🟦️.ts "$GS"/📦️project-records/🧪️tests/📮️retry/🟦️.ts "$N"/📥️take/🧪️tests/🟦️.ts "$N"/📥️take/🧪️tests/🤝️two-machines/🟦️.ts "$N"/🧪️tests/📥️take/🟦️.ts "$N"/🧪️tests/📈️inferred/🟦️.ts "$N"/🧪️tests/🐌️slow-last/🟦️.ts "$N"/🧪️tests/🛑️too-heavy/🟦️.ts "$N"/🧪️tests/🛟️stopped-wave/🟦️.ts "$B"/👁️observation/🧪️tests/🔎️lookup-holds/🟦️.ts "$I"/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts "$I"/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts); fi
for f in $LIST; do
  [ -f "$f" ] || { echo "MISSING ${f#$B/}" >> $SC/$LABEL.txt; continue; }
  out=$($BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
  pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ')
  fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ')
  echo "${f#$B/} :${pass}${fail}" >> $SC/$LABEL.txt
done
echo done >> $SC/$LABEL.txt
