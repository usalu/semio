#!/bin/sh
# usage: run-list.sh <mirror> <list> <out>   runs each "mode file" line serially (mrun.sh), appends to <out>
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while read mode file; do echo "### $file" >> "$3"; sh "$C/red-g-tools/mrun.sh" "$1" "$mode" "$file" --timeout 170000 >> "$3" 2>&1; done < "$2"
echo "### DONE" >> "$3"
