#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
area=caching-cargo
if ! mkdir "$T/landing.lock.$area" 2>/dev/null; then echo "held: $(cat "$T/landing.lock.$area/owner" 2>/dev/null)"; exit 3; fi
echo "H81 $(date +%H:%M:%S)" > "$T/landing.lock.$area/owner"
python3 "$T/🧰️scripts/H81/land.py"; code=$?
rm -f "$T/landing.lock.$area/owner"; rmdir "$T/landing.lock.$area"
exit $code
