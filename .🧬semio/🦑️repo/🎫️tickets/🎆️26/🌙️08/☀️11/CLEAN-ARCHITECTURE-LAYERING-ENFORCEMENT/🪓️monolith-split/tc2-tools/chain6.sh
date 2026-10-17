#!/bin/zsh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tc2-tools
for f in scene edcmd edint host editor model lcmd lcontent lframes b2 b5 transform style structure; do echo "=== $f"; ./orc.sh $f v6 test 2>&1 | /usr/bin/grep -E "^-- cfg"; done
echo CHAIN6-DONE
