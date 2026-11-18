#!/bin/zsh
# R119 landing: run only inside the first 5 minutes of a half hour UTC.
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R119
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
BD=$G/🏗️build; B=$BD/🕸️graph/📥️inference
M=$(date -u +%M); [ $((M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R119 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
LIST=("$B/🧪️tests/🧬️read-verdicts/🟦️.ts" "$B/🧪️tests/🧮️pass-verdicts/🟦️.ts" "$B/🧪️tests/🪦️stale-take/🟦️.ts" "$B/🧪️tests/🪪️layer-ids/🟦️.ts" "$B/🧭️script-inputs/🧪️tests/closure-answer-refresh.test.ts" "$B/🧪️tests/🪤️take-no-derive/🟦️.ts" "$BD/👁️observation/🧪️tests/🤝️adopted-verdict/🟦️.ts" "$BD/👁️observation/🧪️tests/🔂️validation-round/🟦️.ts" "$BD/👁️observation/🧪️tests/⏳️stretch-verdicts/🟦️.ts" "$B/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$B/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts" "$B/📦️project-nodes/🧪️tests/⏳️verdict-wait/🟦️.ts")
one(){ f=$1; for a in 1 2; do out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' '); pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' '); echo "$fail" | grep -qE "[1-9][0-9]* fail" || [ -z "$pass" ] || break; done; echo "${f#$BD/} :$pass$fail (attempt $a)"; }
RES=$(for f in $LIST; do one "$f" & while [ $(jobs -r | wc -l) -ge 4 ]; do sleep 1; done; done; wait)
echo "$RES"
if echo "$RES" | grep -qE "[1-9][0-9]* fail|:\s*\(attempt"; then echo RED; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 4; fi
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
