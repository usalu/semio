#!/bin/zsh
LABEL=$1
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R107
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
BD=$G/🏗️build
B=$BD/🕸️graph/📥️inference
N=$B/📦️project-nodes
LIST=("$N"/🧪️tests/*/🟦️.ts "$N"/📥️take/🧪️tests/*/🟦️.ts "$N"/📥️take/🧪️tests/🟦️.ts "$B"/🧪️tests/🪤️take-no-derive/🟦️.ts "$B"/🧪️tests/🪦️stale-take/🟦️.ts "$B"/🧭️script-inputs/🧪️tests/🫧️take-release/🟦️.ts "$B"/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts "$B"/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts "$BD"/👁️observation/🧪️tests/🔎️lookup-holds/🟦️.ts "$BD"/👁️observation/🧪️tests/⏱️lookup-clock/🟦️.ts "$B"/🧪️tests/🔗️late-closures/🟦️.ts "$B"/🧭️script-inputs/💾️entries/🧪️tests/🟦️.ts "$B"/🧭️script-inputs/💾️entries/🧪️tests/🧰️members-set/🟦️.ts "$B"/🧭️script-inputs/🧪️tests/walk-members-merge.test.ts)
mkdir -p $SC/out $SC/logs; rm -f $SC/out/*
one() {
  f=$1; n=$(echo "$f" | shasum | cut -c1-8)
  for attempt in 1 2 3; do
    out=$($BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
    pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ')
    fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ')
    [ -n "$pass$fail" ] && { echo "$fail" | grep -qE "[1-9][0-9]* fail" || break; }
  done
  echo "$out" | grep -E "^\(fail\)|Expected|Received|error:" | head -20 > $SC/logs/$n.$LABEL.log
  echo "${f#$BD/} :${pass}${fail} (attempt $attempt)" > $SC/out/$n
}
i=0
for f in $LIST; do
  [ -f "$f" ] || { echo "MISSING $f" > $SC/out/m$i; i=$((i+1)); continue; }
  one "$f" &
  i=$((i+1)); (( i % 4 == 0 )) && wait
done
wait
cat $SC/out/* | sort > $SC/$LABEL.txt; echo done >> $SC/$LABEL.txt
