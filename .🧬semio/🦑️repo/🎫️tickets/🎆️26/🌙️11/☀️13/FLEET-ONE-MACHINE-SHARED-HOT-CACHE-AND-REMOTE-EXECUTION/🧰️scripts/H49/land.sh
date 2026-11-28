#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
AREAS=(caching-cargo execution process remote-cache remote-execution)
taken=()
for area in $AREAS; do
  if mkdir "$T/landing.lock.$area" 2>/dev/null; then
    echo "H49 $(date +%H:%M:%S)" > "$T/landing.lock.$area/owner"; taken+=($area)
  else
    echo "held: $area ($(cat "$T/landing.lock.$area/owner" 2>/dev/null))"
    for a in $taken; do rm -f "$T/landing.lock.$a/owner"; rmdir "$T/landing.lock.$a"; done
    exit 3
  fi
done
python3 "$T/🧰️scripts/H49/apply.py"
code=$?
for a in $taken; do rm -f "$T/landing.lock.$a/owner"; rmdir "$T/landing.lock.$a"; done
exit $code
