#!/bin/sh
# usage: runmode.sh <mirror> <mode plain|env|chain> <list> <out>  serial, one block per file
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="${TMPDIR%/}/sm/$1"
while read f; do
  [ -f "$R/$f" ] || { echo "### $f ABSENT" >> "$4"; continue; }
  echo "### $f" >> "$4"; nice -n 10 sh "$C/red-g-tools/mrun.sh" "$1" "$2" "$f" --timeout 170000 >> "$4" 2>&1
done < "$3"
echo "### DONE" >> "$4"
