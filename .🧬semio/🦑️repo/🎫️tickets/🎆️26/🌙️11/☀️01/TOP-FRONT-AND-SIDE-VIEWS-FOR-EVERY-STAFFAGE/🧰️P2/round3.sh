#!/bin/sh
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
O="$T/🗑️generated/P2"
bun "$T/🧰️P2/render.ts" room-thermostat,thermostatic-radiator-valve,vacuum-cleaner,hot-water-cylinder,kitchen-extractor-hood,laptop,mvhr-unit,pendant-lamp,server-rack,shower-head,toaster,split-ac-indoor-unit "$O/r3" 50,100,200 all > "$O/[DEBUG] r3.txt" 2>&1
cd "🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
../../../../../node_modules/.bin/vitest run --config "../../../../../$T/🧰️TS/vitest.three-views.config.ts" > "/c/git/semio/$O/[DEBUG] r3-vitest.txt" 2>&1
echo finished > "/c/git/semio/$O/[DEBUG] r3-done.txt"
