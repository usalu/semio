#!/bin/zsh
# usage: fixloop.sh <family> <from-label-number> <max-rounds>   applies autoimport+autowiden from the previous run, reruns, repeats while something was changed
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F"
fam=$1; n=$2; max=${3:-4}
for r in $(seq 1 $max); do
  prev=r$n; next=r$((n+1))
  ch=0
  for c in lib test; do
    o=$(python3 ohp-tools/autowiden.py $fam $prev $c --apply | tail -1); echo "$fam $prev $c widen: $o"; echo "$o" | /usr/bin/grep -q "widened 0 " || ch=1
    o=$(python3 ohp-tools/autoimport.py $fam $prev $c --apply | tail -1); echo "$fam $prev $c import: $o"; echo "$o" | /usr/bin/grep -q "total imports 0 " || ch=1
    o=$(python3 ohp-tools/autotestkit.py $fam $prev $c --apply | tail -1); echo "$fam $prev $c testkit: $o"; echo "$o" | /usr/bin/grep -q "testkit conversions 0 " || ch=1
  done
  [ $ch = 0 ] && { echo "$fam fixed point at $prev"; break; }
  ./ohp-tools/run.sh $fam $next lib,test
  n=$((n+1))
done
