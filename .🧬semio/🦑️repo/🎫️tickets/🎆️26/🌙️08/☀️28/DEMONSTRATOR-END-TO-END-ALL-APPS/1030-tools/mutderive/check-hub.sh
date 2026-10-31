#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
A="$ROOT/✏️s/🔌️plugins"
cd "$ROOT" || exit 1
run() {
  label=$1; manifest=$2; shift 2
  rm -f "$G/mutderive-$label.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$manifest" --lib --keep-going "$@" > "$G/mutderive-$label.txt" 2>&1
  echo $? > "$G/mutderive-$label.rc"
}
run gen2dtransient "$A/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" -p semio-s-artifact-procedural-2d-editor-transient
run gen3dtransient "$A/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml" -p semio-s-artifact-procedural-3d-viewer-transient
rm -f "$G/mutderive-hub.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/🌎️hub/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-hub-energy-component -p semio-hub-puzzle-component > "$G/mutderive-hub.txt" 2>&1
echo $? > "$G/mutderive-hub.rc"
