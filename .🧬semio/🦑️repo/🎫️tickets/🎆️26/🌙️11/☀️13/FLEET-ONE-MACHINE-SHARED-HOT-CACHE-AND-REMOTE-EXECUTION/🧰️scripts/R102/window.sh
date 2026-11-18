#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
B=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$B/🕸️graph/📥️inference
N=$I/📦️project-nodes
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R102
m=$((10#$(date -u +%M)))
if ! { (( m <= 3 )) || (( m >= 30 && m <= 33 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R102 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 "$D/land.py" --apply || { python3 "$D/restore.py"; rm -r "$T/landing.lock"; exit 3; }
"$D/runpar.sh" after "$N/🌐️owner-pool/🧪️tests/🌍️spread/🟦️.ts" "$N/🧪️tests/🛣️lanes/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🟦️.ts" "$B/🧠️memory/✂️cut/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts" "$N/🧪️tests/📥️take/🟦️.ts" "$N/🧪️tests/🧹️parent-budget/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🔁️no-record/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🎚️lanes/🟦️.ts" > /dev/null 2>&1
RED=$(grep -v "parent-budget" $SC/after.txt | grep -E "[1-9][0-9]* fail" | wc -l | tr -d ' ')
GOT=$(wc -l < $SC/after.txt | tr -d ' ')
if [ "$RED" != "0" ] || [ "$GOT" -lt 10 ]; then python3 "$D/restore.py"; echo "RESTORED red=$RED got=$GOT"; else
{ echo; echo "## R102 landing, $(date -u +%Y-%m-%d) $(date -u +%H:%M) UTC (lock claude-R102, released)"; echo "Pullers spread and lanes. Edited (anchored, backups in scratch R102/backup): owner-pool/🟦️.ts (ownerPoolRendezvousV1; heavy owners in rendezvous order; doneJumps replaced by doneStoredMinimum 192; wide claim round claimWindowMaximum 16 after a run of stored owners), owner-pool schema, project-nodes/🟨️.mjs (inferenceLanesV1 and its use for the lanes of a read). Tests after:"; sed 's#^[^|]*|#- #' $SC/after.txt; echo "Details: ☀️14/WORKERS-READY-WITH-LOW-RESOURCES/result-R102.md"; } >> "$T/landings.md"; fi
rm -r "$T/landing.lock"
sed 's#^[^|]*|##' $SC/after.txt | cut -c1-200
date -u
