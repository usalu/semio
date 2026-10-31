#!/bin/bash
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1030-tools/fem
while [ ! -f "$D/../../🗑️generated/1030/fem-laws.rc" ]; do sleep 20; done
"$D/laws.sh"; cp "$D/../../🗑️generated/1030/fem-laws.txt" "$D/../../🗑️generated/1030/fem-laws2.txt"; cp "$D/../../🗑️generated/1030/fem-laws.rc" "$D/../../🗑️generated/1030/fem-laws2.rc"
FEM_WS="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" "$D/fem.sh" 5 run semio-s-artifact-fem-3d-snapshot-tests
"$D/wasm3d.sh"
