#!/bin/zsh
# H61 landing: authorizer bundle retry. Run ONLY inside a landing slot (:00-:10). Then: landings.md entry "## Wave 1127 H61: ...", repo test-file bundle-settled, repo typecheck, dangling-imports.py.
set -e
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H="$T/🧰️scripts/H61";TOOLS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"
LOCK="$T/landing.lock.bootstrap"
mkdir "$LOCK" || { echo "lock held: $(cat $LOCK/owner)"; exit 1; }
echo "H61 $(date +%T)" > "$LOCK/owner"
trap 'rm -rf "$LOCK"' EXIT
python3 "$H/patch.py" "$TOOLS" --write
cat "$H/test-addition.ts" >> "$TOOLS/🧪️tests/🧱️bundle-settled/🟦️.ts"
