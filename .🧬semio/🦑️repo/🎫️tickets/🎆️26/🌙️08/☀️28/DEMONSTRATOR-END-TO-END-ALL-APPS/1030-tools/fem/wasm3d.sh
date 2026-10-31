#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1030/fem-wasm3d.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/🌎️hub/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-hub-fem-component > "$C/🗑️generated/1030/fem-wasm3d-hub.txt" 2>&1
echo $? > "$C/🗑️generated/1030/fem-wasm3d-hub.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-s-artifact-fem-3d-editor-interaction -p semio-s-artifact-fem-3d-editor-visual-job -p semio-s-artifact-fem-3d-editor-session -p semio-s-artifact-fem-3d-editor > "$C/🗑️generated/1030/fem-wasm3d.txt" 2>&1
echo $? > "$C/🗑️generated/1030/fem-wasm3d.rc"
