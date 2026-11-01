#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
rm -f "$G"/tsdemo2-r4-*.rc
for t in test canonical-architecture test-runtime-catalog; do
  SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:$t > "$G/tsdemo2-r4-$t.txt" 2>&1
  echo $? > "$G/tsdemo2-r4-$t.rc"
done
echo done > "$G/tsdemo2-r4-all.rc"
