#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:$1 > "$C/🗑️generated/1030/e2eprep-$1.txt" 2>&1
echo $? > "$C/🗑️generated/1030/e2eprep-$1.rc"
