#!/bin/zsh
# usage: shape.sh <parentDir>: newcheck + rgi + taxonomy resolution for every child folder directly under parentDir that has a 🟦️.ts or 🦀️.rs
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c6fin2-tools
cd /Users/ueli/Documents/semio; setopt nullglob
P="$1"
ls -d "$P"/*/ | sed 's|/$||' > $C/🗑️generated/fnsplit4b-shape-abs.txt
sed 's|.*/||' $C/🗑️generated/fnsplit4b-shape-abs.txt > $C/🗑️generated/fnsplit4b-shape-names.txt
sed "s|^|/Users/ueli/Documents/semio/|" $C/🗑️generated/fnsplit4b-shape-abs.txt | sed 's|^/Users/ueli/Documents/semio//|/|' > $C/🗑️generated/fnsplit4b-shape-dirs.txt
python3 $C/fnsplit4b-tools/newcheck.py "$P"/*/🟦️.ts "$P"/*/🟦️.tsx "$P"/*/🦀️.rs 2>/dev/null | tail -30
bun $T/rgi.mjs $C/🗑️generated/fnsplit4b-shape-names.txt | tail -3
bun $T/tax.ts $C/🗑️generated/fnsplit4b-shape-dirs.txt 2>&1 | tail -6
