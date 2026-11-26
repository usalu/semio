#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
i=0; : > "$G/ts-status.txt"
while IFS= read -r f; do i=$((i+1)); timeout 900 repo test-file "$f" > "$G/ts-$i.log" 2>&1; echo "$i $? $f" >> "$G/ts-status.txt"; done < "$G/ts-files.txt"
echo done >> "$G/ts-status.txt"
