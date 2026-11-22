#!/bin/sh
# 🔁 Lombardy poplar author: one round = build the candidate, draw its 2D, validate it, then render the sheets named in $LOOKS (lines "name|width|r3-sheet arguments") from the drawn document.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-lombardy-poplar/lombardy-poplar"
python3 rework-lombardy-poplar-model.py | head -6 | cut -c1-260 || exit 1
"$BUN" rework-12-draw2d.ts "$G/pre.json" "$G/plan.json" "$G/🔣️.json" 2>&1 | tail -2
echo "$LOOKS" | while IFS='|' read -r name width args; do [ -n "$name" ] && DOC=🔣️ ./rework-lombardy-poplar-look.sh "$name" "$width" $args; done
"$BUN" r3-validate.ts --file "$G/🔣️.json" $VALIDATE 2>&1 | cut -c1-330 > "$G/validate.txt"
awk '{k=$1" "$2; c[k]++; if(c[k]<=3) print} END{for(k in c) print c[k], k}' "$G/validate.txt" | tail -40
