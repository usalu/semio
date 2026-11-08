#!/bin/bash
# usage: proof.sh <n>  — the host proof for the quiet point: closure check, then the repo test aggregate through the slot.
R=/Users/ueli/Documents/semio; C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; N=$1
cd "$R" || exit 1
export PATH="$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH"
bun "$C/1105b-tools/reacthost/closure-gen.ts" > "$G/reacthost-closure-$N.txt" 2>&1 || { echo $? > "$G/reacthost-$N.rc"; exit 1; }
"$C/1105b-tools/reacthost/run.sh" "$N" @semio-tech/framework-renderer-react:test --run
