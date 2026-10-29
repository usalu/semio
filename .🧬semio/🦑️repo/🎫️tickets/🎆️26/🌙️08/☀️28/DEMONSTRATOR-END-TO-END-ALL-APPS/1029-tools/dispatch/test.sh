#!/bin/bash
# usage: test.sh <N> <crate-dir> <filter>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/dispatch-test$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$2/Cargo.toml" --lib "$3" > "$C/🗑️generated/1029/dispatch-test$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/dispatch-test$1.rc"
