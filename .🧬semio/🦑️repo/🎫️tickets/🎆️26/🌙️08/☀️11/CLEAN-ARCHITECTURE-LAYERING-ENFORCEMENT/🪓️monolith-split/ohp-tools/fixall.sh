#!/bin/zsh
# runs fixloop for every family with a result, from its latest label; waits for the stdio queue first
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F"
while [ ! -f "$F/🗑️generated/ohp-run-stdio34-r1.exit" ]; do sleep 30; done
while [ ! -f "$F/🗑️generated/ohp-run-draw-r2.exit" ]; do sleep 30; done
for fam in "$@"; do
  last=$(ls 🗑️generated/ohp-$fam-r*-result.json 2>/dev/null | sed -E 's/.*-r([0-9]+)-result.json/\1/' | sort -n | tail -1)
  [ -z "$last" ] && continue
  ./ohp-tools/fixloop.sh $fam $last 4
done
echo FIXALL-DONE
