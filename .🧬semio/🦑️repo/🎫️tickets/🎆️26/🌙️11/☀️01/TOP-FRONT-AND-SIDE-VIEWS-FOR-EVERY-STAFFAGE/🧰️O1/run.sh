#!/bin/bash
# usage: run.sh <tag> check|render <ids> [flags]  -> writes 🗑️generated/O1/<tag>.txt
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
tag="$1"; shift
mkdir -p "$T/🗑️generated/O1"
cd /c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage
repo tools exec bun -- "$T/🧰️O1/batch.ts" "$@" 2>&1 | grep -v "^\[capacity\]\|^\[tools\]\|^\[repo\] still" > "$T/🗑️generated/O1/$tag.txt"
echo done >> "$T/🗑️generated/O1/$tag.txt"
