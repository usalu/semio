#!/bin/bash
# usage: run.sh <name> <cargo args after --> 
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
name="$1"; shift
rm -f "$C/🗑️generated/1029/neural-$name.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > "$C/🗑️generated/1029/neural-$name.txt" 2>&1
echo $? > "$C/🗑️generated/1029/neural-$name.rc"
