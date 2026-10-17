#!/bin/zsh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tc2-tools
for f in b2 b5 model lcmd lframes lcontent scene edcmd edint host editor transform style structure; do echo "=== $f"; ./orc.sh $f v4 test 2>&1 | /usr/bin/grep -E "^-- cfg"; done
echo CHAIN4-DONE
