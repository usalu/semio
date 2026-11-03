#!/bin/sh
# usage: run.sh <asset> [scales] [mode]   -> render.ts output in T/🗑️generated/P2/<asset>/
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
mkdir -p "$T/🗑️generated/P2/$1"
repo tools exec bun -- "$T/🧰️P2/render.ts" "$1" "$T/🗑️generated/P2/$1" "${2:-50,100,200}" "${3:-all}" > "$T/🗑️generated/P2/[DEBUG] render-$1.txt" 2>&1
