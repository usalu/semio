#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/hubfix-dt1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/🧰️framework/Cargo.toml" -p semio-framework-value-derive snapshot_projection > "$C/🗑️generated/1029/hubfix-dt1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hubfix-dt1.rc"
