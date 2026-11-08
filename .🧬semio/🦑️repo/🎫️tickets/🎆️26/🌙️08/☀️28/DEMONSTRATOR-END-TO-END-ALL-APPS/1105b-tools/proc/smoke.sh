#!/bin/bash
# usage: smoke.sh <W-label> [en|de] — native generator pane smoke (load example, AddGeneration, undo, close) through the fleet slot semaphore
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
L=${2:-en}; N=$1; O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
M="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=$L repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-procedural-generation3d --test pane-smoke -- generator_pane --nocapture --test-threads=1 > "$O.txt" 2>&1
echo $? > "$O.rc"
