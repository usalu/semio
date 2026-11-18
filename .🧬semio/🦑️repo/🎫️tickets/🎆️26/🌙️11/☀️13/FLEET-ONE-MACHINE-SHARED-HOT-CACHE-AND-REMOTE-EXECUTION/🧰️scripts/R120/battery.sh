#!/bin/bash
# usage: battery.sh <label> [parallel]  runs the R120 test list through the genuine-birth harness, prints one line per file, writes the result to scratch R120/<label>.battery
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=$PWD/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
BD=$G/🏗️build; B=$BD/🕸️graph/📥️inference; PR=$BD/🪪️graph-state/📦️project-records
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120
LIST=("$PR/🚚️entry-lane/🧪️tests/🟦️.ts" "$PR/🚚️entry-lane/🧪️tests/🌊️streamed-intake/🟦️.ts" "$PR/🚚️entry-lane/🧪️tests/🪢️shared-intake/🟦️.ts" "$PR/🧳️portable/🧪️tests/🟦️.ts" "$PR/🧳️portable/🧪️tests/🪶️rest-walk/🟦️.ts" "$PR/🧳️portable/🧪️tests/🔖️document-head/🟦️.ts" "$PR/🧪️tests/🟦️.ts" "$PR/🧪️tests/🚚️closures/🟦️.ts" "$PR/🧪️tests/🗜️packed/🟦️.ts" "$PR/🧪️tests/🌊️seed-stream/🟦️.ts" "$PR/🧪️tests/🤝️pool-reader/🟦️.ts" "$B/🧭️script-inputs/💾️entries/🧪️tests/🟦️.ts" "$B/🧭️script-inputs/🧪️tests/closure-answer-refresh.test.ts" "$B/🧭️script-inputs/🧪️tests/closure-store-limits.test.ts" "$B/🧪️tests/🧬️read-verdicts/🟦️.ts" "$B/🧪️tests/🧮️pass-verdicts/🟦️.ts" "$B/🧪️tests/🪦️stale-take/🟦️.ts" "$B/🧪️tests/🪤️take-no-derive/🟦️.ts" "$B/🧪️tests/🧯️take-no-forced-collection/🟦️.ts" "$B/🧪️tests/🪫️owner-release/🟦️.ts" "$BD/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts" "$B/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$B/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts")
one(){ f=$1; [ -f "$f" ] || { echo "${f#$BD/} :missing"; return; }; for a in 1 2; do out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' '); pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' '); echo "$fail" | grep -qE "[1-9][0-9]* fail" || [ -z "$pass" ] || break; done; echo "${f#$BD/} :$pass$fail (attempt $a)"; }
RES=$(for f in "${LIST[@]}"; do one "$f" & while [ $(jobs -r | wc -l) -ge ${2:-3} ]; do sleep 1; done; done; wait)
echo "$RES" | sort > $SC/$1.battery; cat $SC/$1.battery
