#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1030/testsplit-lock.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- update --workspace --offline > "$C/🗑️generated/1030/testsplit-lock.txt" 2>&1
echo $? > "$C/🗑️generated/1030/testsplit-lock.rc"
