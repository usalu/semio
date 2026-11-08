#!/bin/bash
# usage: build.sh [N] [manifest] [package]   default: flow-list; second proof: build.sh 102 🌎️hub/Cargo.toml semio-hub-energy-component
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N="${1:-101}"; M="${2:-✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml}"; P="${3:-semio-s-plugin-flow-list-component}"
O="$C/🗑️generated/1105b/actinfra-build-$N"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" "W-actinfra-$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- build --manifest-path "$M" --target wasm32-wasip2 --keep-going -p "$P" > "$O.txt" 2>&1
echo $? > "$O.rc"
