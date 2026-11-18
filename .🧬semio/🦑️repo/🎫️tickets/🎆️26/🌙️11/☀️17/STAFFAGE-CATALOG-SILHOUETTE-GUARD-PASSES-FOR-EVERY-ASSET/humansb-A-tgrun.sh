#!/bin/sh
N="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
cd "$N"
cur="$N/humansb-A-u0.json"
for v in front elevation plan; do
  spec=$(python3 humansb-A-autospec.py tour-guide 50 $v flag,pointer,badge)
  python3 humansb-A-greedy.py tour-guide 50 $v 1 "$cur" "$N/🗑️generated/humansb-A-g50-$v.json" c "$spec"
  cur="$N/🗑️generated/humansb-A-g50-$v.json"
done
echo ALLDONE
