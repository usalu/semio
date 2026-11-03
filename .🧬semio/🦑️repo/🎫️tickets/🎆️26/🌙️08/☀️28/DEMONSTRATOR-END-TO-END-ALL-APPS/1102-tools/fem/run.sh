#!/bin/bash
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
rm -f "$C/🗑️generated/1102/$N.rc"
"$C/1102-tools/slot.sh" "W-$N" "$C/1102-tools/fem/test3d.sh" "$N" "$@" > "$C/🗑️generated/1102/$N.slot" 2>&1
