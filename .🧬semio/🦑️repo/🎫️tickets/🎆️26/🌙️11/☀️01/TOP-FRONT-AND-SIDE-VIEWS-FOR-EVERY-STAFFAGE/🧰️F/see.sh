#!/bin/bash
# usage: see.sh <asset> <scale> <name> <queries...>
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
cd /c/git/semio
A=$1; SC=$2; N=$3; shift 3
OUT="$T/🗑️generated/F/$A/$N-$SC.svg"
repo tools exec bun -- "$T/🧰️F/render.ts" "$A" "$SC" "$OUT" "$@" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]"
python "$T/🧰️F/topng.py" "$OUT"
