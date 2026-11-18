#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G="$L/🏗️build/🕸️graph/🕰️generations"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R130
OUT=$1
: > $OUT
FILES=("$G/🧪️tests/🟦️.ts" "$L/🏗️build/🧪️tests/🕸️graph/🧲️owner-claims/🟦️.ts" "$G/🔌️load/🧪️tests/🟦️.ts" "$G/🧪️tests/🗃️row-log/🟦️.ts" "$G/🧪️tests/🌊️stream-patch/🟦️.ts" "$G/🧪️tests/🌊️waves/🟦️.ts" "$G/🧪️tests/🌊️stream/🟦️.ts" "$G/🧪️tests/📋️summary/🟦️.ts" "$G/🧪️tests/⏳️sealing-progress/🟦️.ts" "$G/🧪️tests/🧹️retire/🟦️.ts" "$G/🧪️tests/👯️multiwriter/🟦️.ts" "$G/🧪️tests/🔁️owner-replacement/🟦️.ts" "$G/🧪️tests/🪞️plan-once/🟦️.ts" "$G/🧪️tests/⏱️stages/🟦️.ts" "$G/🔌️load/🧪️tests/🌊️streamed-shape/🟦️.ts" "$G/🧪️tests/🔑️key-table/🟦️.ts" "$G/🧪️tests/🔤️text-arena/🟦️.ts" "$G/🧪️tests/🔖️journal/🟦️.ts" "$G/🧪️tests/🌱️patch-growth/🟦️.ts" "$G/🧪️tests/🌊️flat-memory/🟦️.ts" "$G/🧪️tests/📈️adaptive-waves/🟦️.ts" "$G/🧪️tests/📈️wave-progress/🟦️.ts" "$G/🧪️tests/📋️summary/🟦️.ts" "$G/🧪️tests/🌊️stream/🟦️.ts" "$G/🔌️load/🧪️tests/🔁️carried-release/🟦️.ts")
if [ -n "$R130_SUBSET" ]; then KEEP=(); for f in "${FILES[@]}"; do case "$f" in *"owner-claims"*|*"streamed-shape"*|*"🗃️row-log"*|*"🌊️stream-patch"*|*"🌊️waves"*|*"🔌️load/🧪️tests/🟦️.ts"|*"⏳️sealing-progress"*|*"key-table"*|*"text-arena"*|*"journal"*|*"flat-memory"*|*"carried-release"*|*"📋️summary"*|*"🌊️stream/"*|*"⏱️stages"*|*"adaptive-waves"*|*"wave-progress"*|*"🌱️patch-growth"*|*"🧪️tests/🟦️.ts") KEEP+=("$f");; esac; done; FILES=("${KEEP[@]}"); fi
if [ -n "$R130_PICK" ]; then KEEP=(); for f in "${FILES[@]}"; do for p in ${(s:|:)R130_PICK}; do case "$f" in *"$p"*) KEEP+=("$f"); break;; esac; done; done; FILES=("${KEEP[@]}"); fi
one() {
 f=$1; idx=$2; n=1
 while true; do
  $BUN "$SCRIPT" --file="$f" --birth=setup --root=/Users/ueli/Documents/semio > $SC/bat-$idx.txt 2>&1
  rc=$?
  [ $rc = 0 ] && break
  [ $n -ge 2 ] && break
  n=$((n+1))
 done
 pass=$(grep -E "^ *[0-9]+ (pass|fail)" $SC/bat-$idx.txt | tr -s ' ' | tr '\n' ' ')
 echo "${f#$L/}::rc=$rc::$pass" > $SC/bat-$idx.res
}
idx=0; running=0
for f in "${FILES[@]}"; do
 [ -f "$f" ] || continue
 idx=$((idx+1)); rm -f $SC/bat-$idx.res
 one "$f" $idx &
 running=$((running+1))
 if [ $running -ge ${R130_JOBS:-3} ]; then wait -n 2>/dev/null || wait; running=$((running-1)); fi
done
wait
for k in $(seq 1 $idx); do [ -f $SC/bat-$k.res ] && cat $SC/bat-$k.res >> $OUT; done
