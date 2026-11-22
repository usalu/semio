#!/bin/zsh
# usage: land.sh   takes the six area locks in order, verifies anchors, applies all edits in one step, writes the landing table
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
LIVE=/Users/ueli/Documents/semio
TAKEN=""
AREAS=(caching-bootstrap caching-graph caching-patches caching-tests testing tools)
for a in $AREAS; do
  d="$T/landing.lock.$a"
  if [ -d "$d" ] && [ -n "$(find "$d" -maxdepth 0 -mmin +41 2>/dev/null)" ]; then echo "lock $a older than 40 minutes ($(cat $d/owner 2>/dev/null)): taking over"; rm -f "$d/owner"; rmdir "$d"; fi
  if mkdir "$d" 2>/dev/null; then echo "E28 $(date +%H:%M) nx-split" > "$d/owner"; echo "lock $a taken"; else echo "lock $a HELD: $(cat $d/owner 2>/dev/null)"; for b in $TAKEN; do rm -f "$T/landing.lock.$b/owner"; rmdir "$T/landing.lock.$b"; done; exit 4; fi
  TAKEN="$TAKEN $a"
done
rm -rf /private/tmp/e28/live-base; mkdir -p /private/tmp/e28/live-base
cd "$T/🧰️scripts/E28" || exit 2
python3 lib.py "$LIVE" || { echo "dry run failed"; exit 3; }
python3 lib.py "$LIVE" --apply --base=/private/tmp/e28/live-base > /private/tmp/e28/landing-table.txt || { echo "APPLY FAILED"; exit 5; }
wc -l /private/tmp/e28/landing-table.txt; date
