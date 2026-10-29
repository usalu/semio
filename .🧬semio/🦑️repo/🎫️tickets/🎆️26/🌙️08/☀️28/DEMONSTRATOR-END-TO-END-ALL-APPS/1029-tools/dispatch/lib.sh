#!/bin/bash
# usage: lib.sh <N> <crate-dir relative to repo root>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/dispatch-lib$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$2/Cargo.toml" --lib --keep-going > "$C/🗑️generated/1029/dispatch-lib$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/dispatch-lib$1.rc"
