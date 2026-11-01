#!/bin/zsh
# usage: bt.sh <name> <test file>
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
rm -f "$G/ts2d-$1.rc"
repo tools exec bun -- test "$2" ${3:+-t "$3"} > "$G/ts2d-$1.txt" 2>&1; echo $? > "$G/ts2d-$1.rc"
