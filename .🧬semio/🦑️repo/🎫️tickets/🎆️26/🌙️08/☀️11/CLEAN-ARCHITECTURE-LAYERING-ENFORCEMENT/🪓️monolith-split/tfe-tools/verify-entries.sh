#!/bin/zsh
R=/Users/ueli/Documents/semio
F=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split
cd $R
while IFS=$'\t' read -r a b; do
  [[ "$b" == *"🧪️tests/🧫️fixtures/🟦️.ts" ]] || continue
  echo -n "$b  "
  bun "$F/tfe-tools/entry-probe.ts" "$R/$b" 2>&1 | tail -1
done < <(sort -u -t$'\t' -k2,2 $F/tfe-tools/orig/index.tsv)
