#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
for t in test canonical-architecture test-runtime-catalog; do
  SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:$t > "$C/🗑️generated/1030/e2eprep-$t.txt" 2>&1
  echo $? > "$C/🗑️generated/1030/e2eprep-$t.rc"
done
echo done > "$C/🗑️generated/1030/e2eprep-all.rc"
