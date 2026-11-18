#!/bin/zsh
LABEL=$1; shift
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R102
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
B=$G/🏗️build
I=$B/🕸️graph/📥️inference
N=$I/📦️project-nodes
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
LIST=("$@")
: > $SC/$LABEL.txt
for f in $LIST; do
  [ -f "$f" ] || { echo "MISSING $f" >> $SC/$LABEL.txt; continue; }
  for try in 1 2 3; do
    out=$($BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
    fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ')
    pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ')
    [ -z "$fail" ] || [ "$fail" = " 0 fail" ] && break
  done
  echo "${f##*/📦️project-nodes/}|${f##*/🏗️build/} try=$try :${pass}${fail}" >> $SC/$LABEL.txt
done
echo done >> $SC/$LABEL.txt
