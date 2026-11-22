#!/bin/sh
# 🔁 Small-leaved lime author: one round = build the candidate, draw its 2D, render the sheets named in $LOOKS (lines "name|width|r3-sheet arguments") from the drawn document, validate it ($VALIDATE = extra r3-validate arguments; SKIP_VALIDATE=1 leaves it out; FAST=1 skips the 2D drawing and looks at pre.json; SKIP_BUILD=1 only looks).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-small-leaved-lime/small-leaved-lime"
mkdir -p "$G"
if [ -z "$SKIP_BUILD" ]; then
  python3 rework-small-leaved-lime-model.py > "$G/model.log" 2>&1 || { tail -15 "$G/model.log"; exit 1; }
  cut -c1-260 "$G/model.log" | tail -8
  python3 -c 'import json,sys; p=sys.argv[1]; d=json.load(open(p)); d["elevationCarrierDet"]=0.999999; json.dump(d,open(p,"w"),ensure_ascii=False)' "$G/plan.json"
  [ -n "$FAST" ] || "$BUN" rework-12-draw2d.ts "$G/pre.json" "$G/plan.json" "$G/🔣️.json" 2>&1 | tail -2
fi
DOC="🔣️"; [ -n "$FAST" ] && DOC=pre
echo "$LOOKS" | while IFS='|' read -r name width args; do
  [ -n "$name" ] || continue
  out=$("$BUN" r3-sheet.ts --file "$G/$DOC.json" $args 2>&1 | tail -1)
  [ -f "$out" ] || { echo "FAILED $name $out"; continue; }
  sips -Z "$width" "$out" --out "$G/$name.png" >/dev/null 2>&1; rm -f "$out"; echo "$G/$name.png"
done
[ -n "$SKIP_VALIDATE" ] && exit 0
"$BUN" r3-validate.ts --file "$G/🔣️.json" $VALIDATE 2>&1 | cut -c1-420 > "$G/validate.txt"
awk '{k=$1" "$2; c[k]++; if(c[k]<=3) print} END{for(k in c) print c[k], k}' "$G/validate.txt" | tail -40
