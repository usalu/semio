#!/bin/zsh
# usage: dcheck.sh <rootFileRel> <snapshotName> [nthDescribe]: tsdiff + verify-expand + shape for a dsplit result
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio; setopt nullglob
D="${1%/*}"
node $C/fnsplit4b-tools/verify-expand.cjs "$1" "$C/fnsplit4b-tools/orig/$2" "${3:-0}"
node $C/fnsplit4b-tools/tsdiff.cjs "$1" "$C/fnsplit4b-tools/orig/$2" "$D"/*/🟦️.ts "$D"/*/🟦️.tsx | tail -5
$C/fnsplit4b-tools/shape.sh "$D" 2>&1 | tail -16
