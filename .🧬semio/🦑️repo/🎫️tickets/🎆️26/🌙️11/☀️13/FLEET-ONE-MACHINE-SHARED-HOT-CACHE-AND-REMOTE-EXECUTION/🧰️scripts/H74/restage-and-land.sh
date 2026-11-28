#!/bin/zsh
# H74: restages the repo-land changeset from the current live bases (edits.py), regenerates the help snapshot and lands it with the bootstrap runner. Usage: restage-and-land.sh <scratch H74 dir containing farm2/ and boot.ts>
set -e
S="$1";T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules";B="$M/📚️library/🏗️build";N=$S/stage-final
rm -rf $N;cd /Users/ueli/Documents/semio
nice -n 10 bun run $S/boot.ts --lang=en --lane H74 --title "repo land: changesets land only once they compile (overlay gates, atomic apply, rollback, slots)" checkout $N "$B/⚙️operations/🔣️catalog.json" "$B/⚙️operations/🧬️schema/🔣️.json" "$B/⚙️operations/🧪️tests/🧫️fixtures/🧭️scoping/🔣️.json" "$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json" "$B/🚪️door/🟦️.ts" "$B/🚪️door/🔣️.json" "$B/⌨️command/🧭️selection/🔣️.json" "$B/⌨️command/🧭️selection/🧬️schema/🔣️.json" "$B/🗂️suites/🔣️.json" "$M/🔌️mcp/🖼️assets/🏗️build-surface.json" "$M/🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json"
python3 "$T/H74/edits.py" $N/tree
cp -R "$S/farm2/$B/🚉️land" "$N/tree/$B/🚉️land"
nice -n 10 bun run "$T/H75/help-regen.ts" "/Users/ueli/Documents/semio/$B" "$N/tree/$B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json" "$N/tree/$B/⚙️operations/🔣️catalog.json"
nice -n 10 bun run $S/boot.ts --lang=en --lane H74 --wave "Wave 1127" --rebase $N
