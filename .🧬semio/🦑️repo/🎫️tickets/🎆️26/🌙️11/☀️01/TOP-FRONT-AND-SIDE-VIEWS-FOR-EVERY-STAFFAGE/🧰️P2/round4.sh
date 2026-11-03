#!/bin/sh
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
O="$T/🗑️generated/P2"
mkdir -p "$O/r4"
bun "$T/🧰️P2/render.ts" hot-water-cylinder,kitchen-extractor-hood,laptop,mvhr-unit,pendant-lamp,room-thermostat,server-rack,shower-head,thermostatic-radiator-valve,toaster,vacuum-cleaner,split-ac-indoor-unit "$O/r4" 50,100,200 all > "$O/[DEBUG] r4.txt" 2>&1
bun "$T/🧰️TS/consistency.ts" "$O/[DEBUG] r4-consistency.json" hot-water-cylinder kitchen-extractor-hood laptop mvhr-unit pendant-lamp room-thermostat server-rack shower-head thermostatic-radiator-valve toaster vacuum-cleaner split-ac-indoor-unit > "$O/[DEBUG] r4-consistency.txt" 2>&1
PYTHONUTF8=1 python "$T/🧰️P2/montage.py" "$O/r4" "$O/m4" > "$O/[DEBUG] r4-montage.txt" 2>&1
echo finished > "$O/[DEBUG] r4-done.txt"
