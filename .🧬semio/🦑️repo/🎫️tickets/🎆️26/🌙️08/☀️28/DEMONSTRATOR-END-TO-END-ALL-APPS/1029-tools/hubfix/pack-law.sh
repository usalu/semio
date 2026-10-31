#!/bin/bash
ROOT=/Users/ueli/Documents/semio
cd "$ROOT" || exit 1
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/hubfix-pack.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/🧰️framework/Cargo.toml" -p semio-framework-pack-record-retained --lib original_record_receiving > "$C/🗑️generated/1029/hubfix-pack.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hubfix-pack.rc"
