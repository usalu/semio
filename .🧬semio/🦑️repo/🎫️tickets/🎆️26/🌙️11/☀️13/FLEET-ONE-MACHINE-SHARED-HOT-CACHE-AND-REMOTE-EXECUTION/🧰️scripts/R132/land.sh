#!/bin/bash
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
B="$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
LIB="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
PF="$LIB/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm"
BK=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R132/backup
mkdir "$T/landing.lock" 2>/dev/null || { echo "lock held: $(cat "$T/landing.lock/owner")"; exit 3; }
echo claude-R132 > "$T/landing.lock/owner"
mkdir -p "$BK"
FILES=("🏊️pool/🟨️.mjs" "🏊️pool/🧬️schema/🔣️.json" "🏊️pool/🟨️.d.mts" "🟨️.mjs")
for f in "${FILES[@]}"; do mkdir -p "$BK/$(dirname "$f")"; cp "$PF/$f" "$BK/$f"; done
restore(){ for f in "${FILES[@]}"; do cp "$BK/$f" "$PF/$f"; done; rm -rf "$PF/🧪️tests/⏫️child-priority"; echo RESTORED; rm -rf "$T/landing.lock"; exit 1; }
python3 "$T/🧰️scripts/R132/patch.py" || restore
mkdir -p "$PF/🧪️tests/⏫️child-priority"; cp "$T/🧰️scripts/R132/test.ts" "$PF/🧪️tests/⏫️child-priority/🟦️.ts"
cd "$PF/🧪️tests"
ok=1
for d in ⏫️child-priority 🥇️first-child 🧮️measured-ask 🪦️orphans 🪶️lean 🧮️admission 🛑️hold 🗃️child-custody; do
 out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT "$B" test "./$d/🟦️.ts" --timeout 30000 2>&1)
 echo "$d: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"
 echo "$out" | grep -qE '^ *0 fail' || { ok=0; echo "$out" | tail -30; }
done
for t in "$LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
 cd "$(dirname "$t")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT "$B" test "./🟦️.ts" --timeout 180000 2>&1)
 echo "preflight: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"
 echo "$out" | grep -qE '^ *0 fail' || ok=0
done
[ $ok = 1 ] || restore
echo "## R132 landing (priority class of read farm children), $(date -u +%F\ %T) UTC, lock landing.lock owner claude-R132, released; scripts 🧰️scripts/R132/" >> /dev/null
rm -rf "$T/landing.lock"; echo LANDED
