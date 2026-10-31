#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
rm -f "$C/🗑️generated/1030/prepl-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "${MANIFEST:-Cargo.toml}" --target wasm32-unknown-unknown --keep-going --lib "$@" > "$C/🗑️generated/1030/prepl-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/prepl-$N.rc"
