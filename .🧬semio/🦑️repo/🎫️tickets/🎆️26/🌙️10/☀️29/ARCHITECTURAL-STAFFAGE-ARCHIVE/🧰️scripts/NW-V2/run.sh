#!/bin/bash
# usage: run.sh <asset-module> <asset-id> [scales...]
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
cd "$T/🧰️scripts/NW-V2" && python3 -I "$1_doc.py" >/dev/null || exit 1
id=$2; shift 2
cd "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage" && repo tools exec bun -- "$T/🧰️scripts/NW-V2/suit.ts" "$T/🗑️generated/NW-V2/$id.json" "$@" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]"
