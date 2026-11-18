#!/bin/zsh
setopt nobgnice
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R145
export R145_SCRATCH=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R145
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; done
echo "claude-R145 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
python3 $D/land.py || { python3 $D/restore.py; rm -r "$T/landing.lock"; echo LAND-FAILED; exit 3; }
python3 $D/battery.py all 150 | sed 's#🏗️build/##' > $R145_SCRATCH/battery-after.txt
cat $R145_SCRATCH/battery-after.txt
python3 $D/compare.py $R145_SCRATCH/battery-base.txt $R145_SCRATCH/battery-after.txt || { echo RED; python3 $D/restore.py; rm -r "$T/landing.lock"; exit 4; }
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
