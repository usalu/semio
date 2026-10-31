#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
rm -f "$G/fwfix-lock.rc"
{ SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- update --workspace --offline --manifest-path "🧰️framework/Cargo.toml" && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- metadata --locked --offline --format-version 1 --no-deps --manifest-path "🧰️framework/Cargo.toml" | head -c 300; } > "$G/fwfix-lock.txt" 2>&1
echo $? > "$G/fwfix-lock.rc"
