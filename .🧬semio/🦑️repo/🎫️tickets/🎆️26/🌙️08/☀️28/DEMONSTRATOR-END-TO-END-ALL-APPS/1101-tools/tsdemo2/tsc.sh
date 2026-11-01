#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
repo tools exec tsc -- -p "$C/1030-tools/e2eprep/tsconfig.json" > "$G/tsdemo2-tsc-2.txt" 2>&1; echo $? > "$G/tsdemo2-tsc-2.rc"
