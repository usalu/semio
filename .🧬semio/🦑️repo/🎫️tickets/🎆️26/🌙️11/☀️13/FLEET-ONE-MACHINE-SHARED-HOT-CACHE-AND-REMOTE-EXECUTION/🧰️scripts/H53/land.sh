#!/bin/bash
set -u
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H="$T/🧰️scripts/H53"
W="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden"
take(){ mkdir "$T/landing.lock.$1" 2>/dev/null || { echo "lock $1 held: $(cat "$T/landing.lock.$1/owner" 2>/dev/null)"; return 1; }; echo "H53 $$ $(date +%H:%M:%S)" > "$T/landing.lock.$1/owner"; }
drop(){ rm -rf "$T/landing.lock.$1"; }
take graph || exit 1
python3 "$H/pool-yield.py" land; rc=$?
drop graph
[ $rc -eq 0 ] || exit $rc
take capacity || exit 1
python3 "$H/warden-tree.py" land; rc=$?
if [ $rc -eq 0 ]; then mkdir -p "$W/🧪️tests/🌲️tree"; cp "$H/tests-tree/🟦️.test.ts" "$H/tests-tree/🐍️.py" "$W/🧪️tests/🌲️tree/"; fi
drop capacity
echo "land rc=$rc $(date +%H:%M:%S)"
