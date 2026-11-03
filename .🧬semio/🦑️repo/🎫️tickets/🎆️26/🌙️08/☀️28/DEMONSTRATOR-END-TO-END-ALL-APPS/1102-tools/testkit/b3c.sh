#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"
while [ ! -f $G/b3b-wasm.rc ]; do sleep 20; done
rm -f $G/b3c-*.rc
SEMIO_LANGUAGE=en $S W-b3c-energy repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" --no-fail-fast -p semio-s-artifact-energy-model-editor-mounted-tests --lib > $G/b3c-energy.txt 2>&1; echo $? > $G/b3c-energy.rc
