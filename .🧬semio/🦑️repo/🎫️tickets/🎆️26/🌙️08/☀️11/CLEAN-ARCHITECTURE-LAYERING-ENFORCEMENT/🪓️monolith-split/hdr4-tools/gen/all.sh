#!/bin/zsh
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd /Users/ueli/Documents/semio
python3 "$F/hdr4-tools/pos/pos.py" index --force > /dev/null 2>&1
for fam in "$@"; do
  echo "######## $fam"
  /Users/ueli/.cache/hdr4/tools/run.sh hdr4-$fam fin
done
echo ALLDONE
