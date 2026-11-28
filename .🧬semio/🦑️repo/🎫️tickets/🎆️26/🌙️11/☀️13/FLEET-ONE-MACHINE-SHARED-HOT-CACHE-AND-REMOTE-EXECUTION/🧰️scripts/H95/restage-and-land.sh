#!/bin/zsh
# H95: restages the snapshot + broken-crates changeset from the CURRENT live bases (anchored patches, no stale copies), regenerates the help snapshot,
# runs every gate (`repo land check`) and, with --apply, lands it. Run only after the coordinator lifts the build-system freeze. Usage: restage-and-land.sh <scratch dir> [--apply]
set -e
S="$1";APPLY="$2";H="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";N=$S/stage-h95
cd /Users/ueli/Documents/semio
rm -rf "$N"
nice -n 10 repo land --lane H95 --title "Builds read a consistent source snapshot; broken crates board" checkout "$N" "$B/🚉️land/🔬️gates/🟦️.ts" "$B/🚪️door/🟦️.ts" "$B/🚪️door/🏃️main/🟦️.ts" "$B/🚢️fleet-status/🟦️.ts" "$B/🚢️fleet-status/📥️readers/🟦️.ts" "$B/🚢️fleet-status/⌨️command/🟦️.ts" "$B/🚢️fleet-status/🧬️schema/🔣️.json" "$B/⚙️operations/🔣️catalog.json" "$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
python3 "$H/H95/patch-gates.py" "$N/tree/$B/🚉️land/🔬️gates/🟦️.ts"
python3 "$H/H95/patch-fleet.py" "$N/tree/$B"
for d in 📜️change-journal 📸️snapshot 💥️broken; do mkdir -p "$N/tree/$B/$d"; cp -R "$H/H95/B/$d/." "$N/tree/$B/$d/"; done
nice -n 10 bun run "$H/H75/help-regen.ts" "/Users/ueli/Documents/semio/$B" "$N/tree/$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json" "$N/tree/$B/⚙️operations/🔣️catalog.json"
nice -n 10 repo land check "$N"
[ "$APPLY" = "--apply" ] && nice -n 10 repo land --wave "Wave 1127" --rebase "$N"
