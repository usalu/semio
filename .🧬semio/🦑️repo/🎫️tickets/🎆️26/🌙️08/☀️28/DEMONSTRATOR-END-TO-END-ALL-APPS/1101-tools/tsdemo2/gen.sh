#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
cd /Users/ueli/Documents/semio || exit 1
SEMIO_LANGUAGE=en repo run @semio-tech/framework-rs:generate > "$G/tsdemo2-generate.txt" 2>&1
echo $? > "$G/tsdemo2-generate.rc"
