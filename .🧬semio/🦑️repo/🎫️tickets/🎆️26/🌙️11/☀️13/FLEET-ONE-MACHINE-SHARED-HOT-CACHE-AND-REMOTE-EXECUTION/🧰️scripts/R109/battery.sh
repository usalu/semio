#!/bin/zsh
B=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$B/🕸️graph/📥️inference
N=$I/📦️project-nodes
O=$B/👁️observation/🧪️tests
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts
OUT=${1:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R109/battery}
mkdir -p $OUT; rm -f $OUT/out.*.txt
FILES=("$O/🟦️.ts" "$O/🧾️record-header/🟦️.ts" "$O/📥️reuse/🟦️.ts" "$O/🧠️retention/🟦️.ts" "$O/🧱️blocks/🟦️.ts" "$O/⏱️lookup-clock/🟦️.ts" "$O/🍂️leaf-race/🟦️.ts" "$O/🪜️row-sequence/🟦️.ts" "$O/🧠️file-record-image/🟦️.ts" "$O/🪪️presence/🟦️.ts" "$O/🆔️record-revision/🟦️.ts" ${=EXTRA})
cd /Users/ueli/Documents/semio
i=0
for f in $FILES; do
  [ -f "$f" ] || continue
  i=$((i+1))
  ( out=$($BUN $H --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); echo "$out" | grep -E "^ +[0-9]+ (pass|fail)" | tr -s ' ' | tr '\n' ' ' > $OUT/out.$i.txt; echo "|${f##*/🏗️build/}" >> $OUT/out.$i.txt; echo "$out" > $OUT/full.$i.txt ) &
done
wait
cat $OUT/out.*.txt
