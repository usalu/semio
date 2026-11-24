#!/bin/sh
# Runs the coverage matrix over the stratified sample, two assets per invocation (one per shard), one invocation at a time.
# usage: run-sample.sh <repo-root> <out-dir> [limit-seconds-per-pair]
R="$1"; OUT="$2"; LIMIT="${3:-1800}"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
T="$R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
mkdir -p "$OUT"
: > "$OUT/times.tsv"
cd "$T" || exit 1
while read -r NAME IDS; do
  [ -z "$NAME" ] && continue
  START=$(date +%s)
  timeout "$LIMIT" "$BUN" "📊️matrix.ts" --out "$OUT/$NAME" --jobs 2 --assets "$IDS" > "$OUT/$NAME.log" 2>&1
  CODE=$?
  END=$(date +%s)
  A=$(echo "$IDS" | cut -d, -f1); B=$(echo "$IDS" | cut -d, -f2)
  for I in 0 1; do
    F="$OUT/$NAME/shard-$I.json"
    if [ -f "$F" ]; then M=$(stat -f %m "$F" 2>/dev/null || stat -c %Y "$F"); else M=0; fi
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$NAME" "$I" "$IDS" "$CODE" "$((M - START))" "$((END - START))" >> "$OUT/times.tsv"
  done
done <<LIST
appliances-a electricity-meter,hot-water-cylinder
appliances-b electric-fan-heater,used-tumble-dryer
components-a reclaimed-brick-pallet,used-cast-iron-sectional-boiler
components-b used-district-heating-substation,used-steel-oil-storage-tanks
furniture-a used-coat-stand,used-coffee-table
furniture-b used-high-chair,used-sun-lounger-bench
objects-a used-climbing-net-pyramid,used-wayfinding-signpost
objects-b used-concrete-table-tennis,used-violin
machinery-a finned-heat-exchanger,oil-pumpjack
machinery-b sorption-wheel,wheel-loader
vehicles-a cargo-bike,motor-scooter
vehicles-b tram,vintage-car-1929
humans-a person-with-arm-sling,teenager-with-headphones
humans-b office-worker-carrying-laptop,standing-desk-worker
animals-a hedgehog,red-fox
animals-b mallard-duck,roe-deer
vegetation-a rosemary-bush,weeping-willow
vegetation-b common-ash,red-oak
LIST
echo done > "$OUT/DONE"
