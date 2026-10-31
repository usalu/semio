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
#run energy "$A/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" -p semio-s-artifact-energy-model-mutations
#run bim "$A/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml" -p semio-s-artifact-bim-model-mut-any-56
run gltf "$A/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml" -p semio-s-artifact-stdio-gltf-mutations
run deflate "$A/🗄️stdio/🗿️artifacts/🗜️deflate/Cargo.toml" -p semio-s-artifact-stdio-deflate-mutations -p semio-s-artifact-stdio-deflate
echo done > "$G/mutderive-native.rc"
