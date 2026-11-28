#!/bin/zsh
# H56 landing: handover + sibling seed + their tests under locks command, graph, resident; only locks it created itself
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H=${0:A:h}
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd /Users/ueli/Documents/semio || exit 1
taken=()
release(){ for a in $taken; do rm -f "$T/landing.lock.$a/owner"; rmdir "$T/landing.lock.$a" 2>/dev/null; done; }
for area in command graph resident; do
  if mkdir "$T/landing.lock.$area" 2>/dev/null; then echo "H56 $(date +%T) service handover + sibling memo seed" > "$T/landing.lock.$area/owner"; taken+=($area); else echo "lock $area held: $(cat $T/landing.lock.$area/owner 2>/dev/null)"; release; exit 2; fi
done
DRY=1 python3 $H/apply-handover.py $B && DRY=1 python3 $H/apply-handover-tests.py $B && DRY=1 python3 $H/apply-seed.py $B $H/staged || { release; echo "anchors moved, nothing written"; exit 3; }
python3 $H/apply-handover.py $B && python3 $H/apply-handover-tests.py $B && python3 $H/apply-seed.py $B $H/staged; rc=$?
release
echo "landed rc=$rc $(date +%T)"
