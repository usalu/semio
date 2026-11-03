#!/bin/zsh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS; C=$PWD
SB=/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/sb; rm -rf $SB
for s in "$@"; do echo "== $s"; python3 -I 1102-tools/winplan/sandbox.py $SB/$(basename $s) $C/1102-tools/$s.py 2>&1 | tail -3 | cut -c1-320; done
