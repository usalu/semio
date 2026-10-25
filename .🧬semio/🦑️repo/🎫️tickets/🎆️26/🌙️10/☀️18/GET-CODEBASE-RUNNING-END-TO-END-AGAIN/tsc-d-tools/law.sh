#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/tsc-d"
tag="$1"; shift
rm -f "$G/$tag.done"
start=$(date +%s)
nice -n 10 "$@" > "$G/$tag.txt" 2>&1
rc=$?
echo "$tag rc=$rc secs=$(( $(date +%s)-start ))" > "$G/$tag.done"
