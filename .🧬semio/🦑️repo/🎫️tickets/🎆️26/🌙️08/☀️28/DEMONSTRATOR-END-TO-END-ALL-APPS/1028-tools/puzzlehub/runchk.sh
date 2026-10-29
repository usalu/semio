#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
N="$1"; shift
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check "$@" > "$G/puzzlehub-chk$N.txt" 2>&1
echo $? > "$G/puzzlehub-chk$N.rc"
