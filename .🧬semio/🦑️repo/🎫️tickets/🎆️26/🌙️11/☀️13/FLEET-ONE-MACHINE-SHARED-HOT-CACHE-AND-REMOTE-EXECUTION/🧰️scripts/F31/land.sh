#!/bin/zsh
# F31 landing, run from /Users/ueli/Documents/semio after 19:00 with the locks graph, bootstrap, tools taken (alphabetical).
set -u
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S="$T/🧰️scripts/F31"
python3 "$S/apply.py" || exit 3
repo ./📜️script.ts verify build-owners sync || exit 4
python3 "$S/scan-ownerless.py" | wc -l
