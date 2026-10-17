#!/bin/zsh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tc2-tools
for f in host edcmd edint editor; do echo "=== $f"; ./orc.sh $f v8 test 2>&1 | /usr/bin/grep -E "^-- cfg"; done
echo CHAIN9-DONE
