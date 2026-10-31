#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1030/fem-model$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --keep-going --manifest-path "$ROOT/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" -p semio-s-artifact-fem-3d-model > "$C/🗑️generated/1030/fem-model$1.txt" 2>&1
echo $? > "$C/🗑️generated/1030/fem-model$1.rc"
