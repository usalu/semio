#!/bin/sh
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
O="$T/🗑️generated/P2"
bun "$T/🧰️P2/render.ts" hot-water-cylinder,kitchen-extractor-hood,laptop,mvhr-unit,pendant-lamp,room-thermostat,server-rack,shower-head,thermostatic-radiator-valve,toaster,vacuum-cleaner,split-ac-indoor-unit "$O/x" 50,100,200 extents > "$O/[DEBUG] final-check.txt" 2>&1
bun "$T/🧰️TS/consistency.ts" "" hot-water-cylinder kitchen-extractor-hood laptop mvhr-unit pendant-lamp room-thermostat server-rack shower-head thermostatic-radiator-valve toaster vacuum-cleaner split-ac-indoor-unit > "$O/[DEBUG] final-consistency.txt" 2>&1
cd "🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
../../../../../node_modules/.bin/vitest run --config "../../../../../$T/🧰️TS/vitest.three-views.config.ts" > "/c/git/semio/$O/[DEBUG] final-vitest.txt" 2>&1
echo finished > "/c/git/semio/$O/[DEBUG] final-done.txt"
