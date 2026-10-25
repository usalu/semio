#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nx4-d"
target="$1"; name="$2"
echo "start=$(date +%H:%M:%S) pid=$$" > "$G/$name.log"
nice -n 10 bun repo --tool=nx --lang=en inputs "$target" > "$G/$name.json" 2>> "$G/$name.log"
echo "rc=$? end=$(date +%H:%M:%S)" >> "$G/$name.log"
