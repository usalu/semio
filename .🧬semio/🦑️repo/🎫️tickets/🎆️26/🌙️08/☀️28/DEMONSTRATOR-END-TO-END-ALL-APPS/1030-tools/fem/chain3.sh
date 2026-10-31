#!/bin/bash
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS
G="$D/🗑️generated/1030"; T="$D/1030-tools/fem"
"$T/laws.sh"; cp "$G/fem-laws.txt" "$G/fem-laws5.txt"; cp "$G/fem-laws.rc" "$G/fem-laws5.rc"
FEM_WS="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" "$T/fem.sh" 7 run semio-s-artifact-fem-3d-snapshot-tests
"$T/wasm3d.sh"; cp "$G/fem-wasm3d-hub.txt" "$G/fem-wasm3d-hub3.txt"; cp "$G/fem-wasm3d-hub.rc" "$G/fem-wasm3d-hub3.rc"; cp "$G/fem-wasm3d.txt" "$G/fem-wasm3d2.txt"; cp "$G/fem-wasm3d.rc" "$G/fem-wasm3d2.rc"
