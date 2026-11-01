#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
P=✏️s/🔌️plugins
bash $C/1101-tools/chk.sh "$1-gen3" $P/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml wasm -p semio-s-artifact-procedural-3d-mode-generate
bash $C/1101-tools/chk.sh "$1-gen2" $P/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml wasm -p semio-s-artifact-procedural-2d-mode-generate
bash $C/1101-tools/chk.sh "$1-fem" $P/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml wasm -p semio-s-artifact-fem-3d-editor -p semio-s-artifact-fem-2d-editor
bash $C/1101-tools/chk.sh "$1-femtests" $P/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml native --tests -p semio-s-artifact-fem-3d-editor-results-window -p semio-s-artifact-fem-2d-editor-results-window
