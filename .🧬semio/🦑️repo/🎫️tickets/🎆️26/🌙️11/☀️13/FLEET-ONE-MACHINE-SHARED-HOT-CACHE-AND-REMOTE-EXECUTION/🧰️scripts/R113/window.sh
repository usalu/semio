#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R113
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R113 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
echo "start $(date -u)"
(cd "$D" && $BUN land.mjs) || { (cd "$D" && $BUN restore.mjs); rm -r "$T/landing.lock"; exit 3; }
"$D/battery.sh" after
python3 "$D/compare.py" before after && echo GREEN || { echo RED; (cd "$D" && $BUN restore.mjs); }
rm -r "$T/landing.lock"
cat $SC/after.txt
echo "end $(date -u)"
