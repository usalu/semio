#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
cd "$ROOT" || exit 1
rm -f "$G/mutderive-homefold.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.toml" --lib -p semio-s-artifact-space-home-editor -- the_fold_stepper > "$G/mutderive-homefold.txt" 2>&1
echo $? > "$G/mutderive-homefold.rc"
