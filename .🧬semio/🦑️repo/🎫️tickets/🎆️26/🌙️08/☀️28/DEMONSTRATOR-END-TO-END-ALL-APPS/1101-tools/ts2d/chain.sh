#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
for t in "$@"; do
  n=$(echo "$t" | tr '/:@' '___')
  rm -f "$G/ts2d-host-$n.rc"
  SEMIO_LANGUAGE=en repo run "$t" > "$G/ts2d-host-$n.txt" 2>&1; echo $? > "$G/ts2d-host-$n.rc"
done
echo done > "$G/ts2d-host-all.rc"
