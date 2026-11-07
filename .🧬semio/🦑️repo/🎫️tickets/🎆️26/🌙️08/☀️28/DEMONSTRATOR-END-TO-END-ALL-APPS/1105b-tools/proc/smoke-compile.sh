#!/bin/bash
# usage: smoke-compile.sh <W-label> — compile only (--no-run) the generator pane smoke test tree
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
M="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-procedural-generation3d --test pane-smoke --no-run > "$O.txt" 2>&1
echo $? > "$O.rc"
