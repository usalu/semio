#!/bin/zsh
# H83 landing: locks fleet-status + operations (alphabetical), anchored atomic patches, help snapshot regeneration, release.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H="$T/🧰️scripts/H83"
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H83
FS="$B/🚢️fleet-status"
for area in fleet-status operations; do
  waited=0
  until mkdir "$T/landing.lock.$area" 2>/dev/null; do
    sleep 5; waited=$((waited+5))
    if [ $waited -ge 240 ]; then echo "lock $area busy"; echo "H83 $(date +%H:%M) waiting" >> "$T/landing.queue.$area"; for a in fleet-status operations; do [ -f "$T/landing.lock.$a/owner" ] && grep -q H83 "$T/landing.lock.$a/owner" && rm -r "$T/landing.lock.$a"; done; exit 3; fi
  done
  echo "H83 $(date +%H:%M)" > "$T/landing.lock.$area/owner"
done
trap 'for a in operations fleet-status; do [ -d "$T/landing.lock.$a" ] && rm -r "$T/landing.lock.$a"; done' EXIT
set -e
python3 "$H/schema.py" "$FS/🧬️schema/🔣️.json"
python3 "$H/apply.py" "$FS" "$B/⚙️operations/🔣️catalog.json" "$H/block.ts"
python3 "$H/tests.py" "$FS" "$H/tests-append.ts"
nice -n 10 bun "$T/🧰️scripts/H75/help-regen.ts" "$B" "$S/help-new.json" "$B/⚙️operations/🔣️catalog.json"
cp "$S/help-new.json" "$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json.h83.tmp" && mv "$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json.h83.tmp" "$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
echo landed $(date +%H:%M:%S)
