#!/bin/sh
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
repo tools exec bun -- "$T/🧰️P2/render.ts" hot-water-cylinder,kitchen-extractor-hood,laptop,mvhr-unit,pendant-lamp,room-thermostat,server-rack,shower-head,thermostatic-radiator-valve,toaster,vacuum-cleaner,split-ac-indoor-unit "$T/🗑️generated/P2/x" 50,100,200 ${1:-all} > "$T/🗑️generated/P2/[DEBUG] all.txt" 2>&1
