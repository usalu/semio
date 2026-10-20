#!/bin/zsh
# usage: check-live.sh : after `staged/apply.py --write`: anchors, shape law and taxonomy resolution on the LIVE tree (read-only)
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
TD=$C/fnsplit4a-finish-tools
cd /Users/ueli/Documents/semio
python3 $TD/staged/apply.py --verify
nice -n 10 bun $TD/shape.mjs --depth 1 $(cat $TD/dirs11.txt | tr '\n' ' ')
while IFS='|' read -r d o; do nice -n 10 bun $TD/shape.mjs --depth 1 --only "$o" "$d"; done < $TD/shape-rows.txt
