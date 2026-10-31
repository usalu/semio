#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
cd "$ROOT" || exit 1
rm -f "$G/mutderive-gen3dtransient.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" --lib --keep-going -p semio-s-artifact-procedural-3d-viewer-transient -p semio-s-artifact-procedural-3d-editor-transient > "$G/mutderive-gen3dtransient.txt" 2>&1
echo $? > "$G/mutderive-gen3dtransient.rc"
