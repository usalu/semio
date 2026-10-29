#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
N="$1"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml" --target wasm32-unknown-unknown --keep-going -p semio-hub-puzzle > "$G/puzzlehub-hub$N.txt" 2>&1
echo $? > "$G/puzzlehub-hub$N.rc"
