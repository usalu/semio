#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
cd "$ROOT" || exit 1
while [ ! -f "$G/mutderive-gen3dtransient.rc" ]; do sleep 20; done
run() { label=$1; verb=$2; manifest=$3; shift 3; rm -f "$G/mutderive-$label.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- $verb --manifest-path "$manifest" --lib "$@" > "$G/mutderive-$label.txt" 2>&1; echo $? > "$G/mutderive-$label.rc"; }
run fem2dtransient check "$ROOT/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" --keep-going -p semio-s-artifact-fem-2d-editor-transient
run hashtest test "$ROOT/🧰️framework/Cargo.toml" -p semio-framework-replication-mutation -- roster_uniqueness_hashes
rm -f "$G/mutderive-hub.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/🌎️hub/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-hub-energy-component -p semio-hub-puzzle-component > "$G/mutderive-hub.txt" 2>&1
echo $? > "$G/mutderive-hub.rc"
echo done > "$G/mutderive-chain2.rc"
