#!/bin/zsh
setopt nobgnice
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R136
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R136
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; done
echo "claude-R136 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
python3 $D/land.py || { python3 $D/restore.py; rm -r "$T/landing.lock"; echo LAND-FAILED; exit 3; }
mkdir -p "$D/🗑️generated/art-after"
ART="$D/🗑️generated/art-after" $D/battery.sh $SC/after.txt base $D/inlock.txt 8
python3 $D/compare.py $SC/base.txt $SC/after.txt || { echo RED-compare; python3 $D/restore.py; rm -r "$T/landing.lock"; exit 4; }
awk '/requested-group|slice-lab|wide-failure/ && ($NF!=0 || $(NF-1)<1){bad=1} END{exit bad}' $SC/after.txt || { echo RED-new; python3 $D/restore.py; rm -r "$T/landing.lock"; exit 5; }
sed 's#^.*📚️library/##' $SC/after.txt
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
