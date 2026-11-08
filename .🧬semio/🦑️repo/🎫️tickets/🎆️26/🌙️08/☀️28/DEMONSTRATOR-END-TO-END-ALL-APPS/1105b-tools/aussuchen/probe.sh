#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
M="✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-sourcing-curation --test aussuchen_pane_smoke -- aussuchen_ --nocapture > "$O.txt" 2>&1
echo $? > "$O.rc"
