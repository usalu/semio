#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
N=$1; shift
rm -f "$G/p2dpix-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$@" > "$G/p2dpix-$N.txt" 2>&1
echo $? > "$G/p2dpix-$N.rc"
