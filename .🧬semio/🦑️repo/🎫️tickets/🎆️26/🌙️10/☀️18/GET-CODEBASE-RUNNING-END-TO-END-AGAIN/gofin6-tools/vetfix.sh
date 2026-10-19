#!/usr/bin/env bash
# usage: vetfix.sh <slug> <module dir relative to repo root> -- vet, then drop unused imports in the REAL sources (the copy is resynced by the next call)
REAL=/Users/ueli/Documents/semio
C="$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out=$(HEADN=200 bash "$C/gofin6-tools/vet-mod.sh" "$1" "$2")
echo "$out" | python3 "$C/gofin6-tools/fiximports.py" "$REAL/$2"
echo "$out" | /usr/bin/grep -v 'imported and not used' | head -${SHOWN:-40}
