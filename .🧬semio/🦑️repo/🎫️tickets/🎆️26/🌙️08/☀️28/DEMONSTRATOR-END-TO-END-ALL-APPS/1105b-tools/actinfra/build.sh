#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1105b/actinfra-build"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" W-actinfra-build env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- build --manifest-path ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml --target wasm32-wasip2 --keep-going -p semio-s-plugin-flow-list-component > "$O.txt" 2>&1
echo $? > "$O.rc"
