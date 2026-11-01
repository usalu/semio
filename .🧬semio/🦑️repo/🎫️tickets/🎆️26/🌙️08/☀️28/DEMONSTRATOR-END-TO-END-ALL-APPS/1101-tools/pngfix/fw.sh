#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1101/pngfix-fw"; rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path 🧰️framework/Cargo.toml -p semio-framework-actor-retained-turn --lib > "$O.txt" 2>&1
echo $? > "$O.rc"
