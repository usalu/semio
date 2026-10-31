#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
cd "$ROOT" || exit 1
rm -f "$G/mutderive-retained.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/🧰️framework/Cargo.toml" --lib --keep-going -p semio-framework-replication-mutation-retained > "$G/mutderive-retained.txt" 2>&1
echo $? > "$G/mutderive-retained.rc"
