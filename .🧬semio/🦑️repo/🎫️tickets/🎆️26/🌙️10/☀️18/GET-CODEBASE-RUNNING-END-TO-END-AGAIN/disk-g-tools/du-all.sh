#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/disk-g"
for p in p d g o n; do nice -n 15 du -k -x -d 2 .cache/b/$p/m > "$G/du-$p.txt" 2>/dev/null; done
nice -n 15 du -k -x -d 2 ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m" > "$G/du-rust.txt" 2>/dev/null
echo finished > "$G/du-all.done"
