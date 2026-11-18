#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R97
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R97 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 "$D/land.py" || { python3 "$D/restore.py"; rm -r "$T/landing.lock"; exit 3; }
"$D/runall.sh" after
if grep -E "MISSING|[1-9][0-9]* fail" $SC/after.txt | grep -v "(attempt [12])" > /dev/null; then echo RED; python3 "$D/restore.py"; fi
rm -r "$T/landing.lock"
cat $SC/after.txt
date -u
