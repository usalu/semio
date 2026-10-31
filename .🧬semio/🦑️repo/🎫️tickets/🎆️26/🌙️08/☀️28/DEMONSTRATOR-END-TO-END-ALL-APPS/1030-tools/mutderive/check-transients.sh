#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
A="$ROOT/✏️s/🔌️plugins"
cd "$ROOT" || exit 1
run() { label=$1; manifest=$2; shift 2; rm -f "$G/mutderive-$label.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$manifest" --lib --keep-going "$@" > "$G/mutderive-$label.txt" 2>&1; echo $? > "$G/mutderive-$label.rc"; }
rm -f "$G/mutderive-transients.rc"
run wfc2dtransient "$A/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml" -p semio-s-artifact-wfc-2d-transient
run wfc3dtransient "$A/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml" -p semio-s-artifact-wfc-3d-transient
run wfcbitmaptransient "$A/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml" -p semio-s-artifact-wfc-bitmap-transient
run fem2dtransient "$A/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" -p semio-s-artifact-fem-2d-editor-transient
echo done > "$G/mutderive-transients.rc"
