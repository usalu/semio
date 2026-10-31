#!/bin/bash
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS
G="$D/🗑️generated/1030"; T="$D/1030-tools/fem"
while [ ! -f "$G/fem-6.rc" ]; do sleep 20; done
"$T/laws.sh"; cp "$G/fem-laws.txt" "$G/fem-laws4.txt"; cp "$G/fem-laws.rc" "$G/fem-laws4.rc"
"$T/wasm3d.sh"; cp "$G/fem-wasm3d-hub.txt" "$G/fem-wasm3d-hub2.txt"; cp "$G/fem-wasm3d-hub.rc" "$G/fem-wasm3d-hub2.rc"
