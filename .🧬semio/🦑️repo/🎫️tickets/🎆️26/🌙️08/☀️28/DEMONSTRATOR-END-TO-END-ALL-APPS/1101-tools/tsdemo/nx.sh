#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_LANGUAGE=en
for t in test canonical-architecture test-runtime-catalog; do
  repo --tool=nx run @semio-tech/mit-bestand-demonstrator:$t > "$G/tsdemo-nx-$t.txt" 2>&1; echo $? > "$G/tsdemo-nx-$t.rc"
done
echo done > "$G/tsdemo-nx-all.rc"
