#!/bin/zsh
# usage: cycle.sh <test paths relative to library root...>   reset+apply edits to scratch, run tests
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd "$T/🧰️scripts/E28" && python3 lib.py /private/tmp/e28/co --reset --base=/private/tmp/e28/base && python3 lib.py /private/tmp/e28/co --apply --base=/private/tmp/e28/base >/dev/null || exit 3
cd /private/tmp/e28/co
for f in "$@"; do
  echo "== ${f}"
  repo test-file --json "$L/$f" 2>&1 | grep -E '^\(fail|error:|^ *[0-9]+ (pass|fail)|"status"' | head -${LINES_MAX:-14}
done
