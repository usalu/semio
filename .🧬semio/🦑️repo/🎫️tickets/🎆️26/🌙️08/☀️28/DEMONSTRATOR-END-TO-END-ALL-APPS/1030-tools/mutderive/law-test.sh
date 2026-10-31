#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
A="$ROOT/✏️s/🔌️plugins"
cd "$ROOT" || exit 1
run() {
  label=$1; manifest=$2; shift 2
  rm -f "$G/mutderive-law-$label.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$manifest" --lib "$@" > "$G/mutderive-law-$label.txt" 2>&1
  echo $? > "$G/mutderive-law-$label.rc"
}
run epw "$A/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.toml" -p semio-s-artifact-stdio-epw-mutations -- semio_payload_law
run gltf "$A/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml" -p semio-s-artifact-stdio-gltf-mutations -- semio_payload_law
run energy "$A/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" -p semio-s-artifact-energy-model-mutations -- semio_payload_law
echo done > "$G/mutderive-law.rc"
