#!/bin/zsh
LABEL=$1; shift
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R102
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts
rm -f $SC/$LABEL.*.txt; i=0
for f in "$@"; do
  i=$((i+1))
  ( out=$($BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); echo "${f##*/🏗️build/}| $(echo "$out" | grep -E '^ +[0-9]+ pass' | tail -1 | tr -s ' ')$(echo "$out" | grep -E '^ +[0-9]+ fail' | tail -1 | tr -s ' ')" > $SC/$LABEL.$i.txt; echo "$out" > $SC/$LABEL.$i.full ) &
done
wait
cat $SC/$LABEL.*.txt > $SC/$LABEL.txt
