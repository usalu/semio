#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R100
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R100 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
date -u +%H:%M:%S
python3 "$D/land.py" || { python3 "$D/restore.py"; rm -r "$T/landing.lock"; echo "land failed, restored, lock released"; exit 3; }
"$D/batch.sh" after > /dev/null 2>&1
cat $SC/after.txt
date -u +%H:%M:%S
echo "lock still held: run release.sh (green) or restore.sh (red)"
