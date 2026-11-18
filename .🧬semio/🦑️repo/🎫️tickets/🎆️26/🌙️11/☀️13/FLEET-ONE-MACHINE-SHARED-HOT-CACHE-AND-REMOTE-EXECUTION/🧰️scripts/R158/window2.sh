#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R158
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R158
until [ $((10#$(date -u +%M))) -lt 5 ]; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R158 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land2.py restore $SCR/bk2; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk2 $SCR/after2
python3 $D/land2.py apply $SCR/bk2 || fail apply
$D/battery2.sh $SCR/after2
python3 $D/compare.py $SCR/base2/summary.txt $SCR/after2/summary.txt || fail tests
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
