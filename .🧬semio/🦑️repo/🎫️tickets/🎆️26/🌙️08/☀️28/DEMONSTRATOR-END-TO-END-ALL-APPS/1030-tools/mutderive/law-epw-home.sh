#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
A="$ROOT/✏️s/🔌️plugins"
cd "$ROOT" || exit 1
rm -f "$G/mutderive-lawepw.rc" "$G/mutderive-homefold.rc" "$G/mutderive-lawhome.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$A/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.toml" --lib -p semio-s-artifact-stdio-epw-mutations -- semio_payload_law > "$G/mutderive-lawepw.txt" 2>&1
echo $? > "$G/mutderive-lawepw.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$A/🪐️space/🗿️artifacts/🏠️home/Cargo.toml" --lib --keep-going -p semio-s-artifact-space-home-editor -- the_fold_stepper > "$G/mutderive-homefold.txt" 2>&1
echo $? > "$G/mutderive-homefold.rc"
echo done > "$G/mutderive-lawhome.rc"
