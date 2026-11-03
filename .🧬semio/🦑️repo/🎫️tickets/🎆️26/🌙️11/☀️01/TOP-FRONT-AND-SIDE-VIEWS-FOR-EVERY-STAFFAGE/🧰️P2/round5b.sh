#!/bin/sh
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
O="$T/🗑️generated/P2"
bun "$T/🧰️P2/render.ts" laptop "$O/x" 50,100,200 extents > "$O/[DEBUG] r5b-check.txt" 2>&1
bun "$T/🧰️P2/crops.ts" "$O/c5b" > "$O/[DEBUG] r5b-crops.txt" 2>&1
echo f > "$O/[DEBUG] r5b-done.txt"
