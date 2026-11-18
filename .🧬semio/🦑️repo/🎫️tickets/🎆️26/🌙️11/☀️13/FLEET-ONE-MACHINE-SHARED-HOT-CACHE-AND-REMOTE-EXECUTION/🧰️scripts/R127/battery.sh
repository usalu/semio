#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G="$L/🏗️build/🕸️graph/🕰️generations"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
OUT=$1
: > $OUT
FILES=("$G/🧪️tests/🟦️.ts")
for f in $G/🧪️tests/*/🟦️.ts; do FILES+=("$f"); done
FILES+=("$L/🏗️build/🕸️graph/🧪️tests/🟦️.ts")
if [ -n "$R127_SUBSET" ]; then KEEP=(); for f in "${FILES[@]}"; do case "$f" in *"⏳️sealing-progress"*|*"📋️summary"*|*"📈️wave-progress"*|*"🌊️waves"*|*"🌊️stream/"*|*"🧹️retire"*|*"👯️multiwriter"*|*"🔁️owner-replacement"*|*"🍂️view-release"*|*"♻️refine"*|*"🪞️plan-once"*|*"⏱️stages"*|*"🏷️config-owners"*|*"🧪️tests/🟦️.ts") KEEP+=("$f");; esac; done; FILES=("${KEEP[@]}"); fi
for f in "${FILES[@]}"; do
 [ -f "$f" ] || continue
 case "$f" in *"$R127_SKIP"*) [ -n "$R127_SKIP" ] && continue;; esac
 n=1
 while true; do
  $BUN "$SCRIPT" --file="$f" --birth=setup --root=/Users/ueli/Documents/semio > /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R127/bat.txt 2>&1
  rc=$?
  [ $rc = 0 ] && break
  [ $n -ge 2 ] && break
  n=$((n+1))
 done
 pass=$(grep -E "^ *[0-9]+ pass" /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R127/bat.txt | tail -1 | tr -s ' ')
 echo "${f#$L/}::rc=$rc::$pass" >> $OUT
done
echo done >> $OUT
