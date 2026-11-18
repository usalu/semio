#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R139
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R139
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R139 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
fail(){ echo "RED: $1"; python3 $D/land.py restore; rm -r "$T/landing.lock"; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SC/backup
python3 $D/land.py apply || fail apply
rm -f $SC/lock.*
i=0; while read d; do i=$((i+1)); ( $SC/t.sh base "$d" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$" > $SC/lock.$i ) & while [ $(jobs -r | wc -l) -ge 4 ]; do sleep 1; done; done < $D/inlock.txt; wait
bad=0; i=0; while read d; do i=$((i+1)); n=$(awk '/ fail$/{print $1}' $SC/lock.$i); allowed=0; echo "$d $(tr '\n' ' ' < $SC/lock.$i | cut -c1-200)"; [ "${n:-9}" -le $allowed ] || bad=1; done < $D/inlock.txt
[ $bad = 0 ] || fail tests
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
