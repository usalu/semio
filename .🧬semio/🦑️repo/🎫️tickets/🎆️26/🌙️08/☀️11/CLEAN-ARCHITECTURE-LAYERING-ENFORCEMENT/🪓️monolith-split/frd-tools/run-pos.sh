#!/bin/zsh
# usage: run-pos.sh <family-name>...   (sequential, POS_PREFIX=frd; logs frd-pos-<family>.out/.err)
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
export POS_PREFIX=frd
ulimit -n 65536
for fam in "$@"; do
  perl -e 'alarm 3000; exec @ARGV' -- nice -n 10 python3 frd-tools/pos/pos.py run "frd-tools/families/$fam.json" --cfgs lib,test --no-base > "🗑️generated/frd-pos-$fam.out" 2> "🗑️generated/frd-pos-$fam.err"
  echo "$fam done $?" >> "🗑️generated/frd-pos-queue.log"
done
