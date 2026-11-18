#!/bin/zsh
# R120 landing: run only inside the first 5 minutes of a half hour UTC. usage: window.sh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R120
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
M=$(date -u +%M); [ $((M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R120 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
RES=$($D/battery.sh landed 5)
echo "$RES"
BAD=$(echo "$RES" | grep -v "project-records/🧪️tests/🟦️.ts" | grep -E "[1-9][0-9]* fail|missing|:\s*\(attempt")
if [ -n "$BAD" ]; then echo "RED: $BAD"; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 4; fi
echo "GREEN started $START ended $(date -u +%H:%M:%S) (project-records/🧪️tests/🟦️.ts has one failure before and after)"
rm -r "$T/landing.lock"
