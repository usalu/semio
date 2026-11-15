#!/bin/sh
# [scratch] takes the landing lock as soon as it is free (up to $1 seconds) and applies the Package E patches to the live files
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
LAND=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/E/land
LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
n=0
until mkdir "$T/landing.lock" 2>/dev/null; do
  n=$((n+1)); [ $n -ge $1 ] && { echo "lock busy: $(cat "$T/landing.lock/owner")"; exit 2; }
  sleep 1
done
echo "claude-package-E $(date +%s) $(date)" > "$T/landing.lock/owner"
echo "lock taken"
FILES="🏗️build/🌐️remote-execution/🔒️sandbox/🟦️.ts 🏗️build/🌐️remote-execution/👷️worker/🟦️.ts 🏗️build/🌐️remote-execution/⌨️command/🟦️.ts ⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"
for f in $FILES; do echo "before $(shasum -a 256 "$LIB/$f" | cut -c1-12) $f"; done > "$LAND/before.txt"
RE="$LIB/🏗️build/🌐️remote-execution"
python3 "$LAND/worker.py" "$RE" && python3 "$LAND/command.py" "$RE" && python3 "$LAND/wrapper.py" "$LIB/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts" || { echo "PATCH FAILED"; exit 3; }
for f in $FILES; do echo "after $(shasum -a 256 "$LIB/$f" | cut -c1-12) $f"; done > "$LAND/after.txt"
cat "$LAND/before.txt" "$LAND/after.txt"
