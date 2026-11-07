#!/bin/bash
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1105b"
for f in $(ls "$G" | /usr/bin/grep "^W-ext-4-.*\.rc$"); do echo "$f $(cat "$G/$f")"; done
echo "started: $(ls "$G" | /usr/bin/grep -c '^W-ext-4-.*\.txt$')"
