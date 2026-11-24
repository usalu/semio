#!/bin/zsh
# 👀️ The product drawings of every asset of batch SAV before (backup of the state before this fixer) and after (catalog file) at 1:50 and at its coarsest authored scale (`sheets.sh`), stacked three assets to an image at reduced width for the review (`stack.ts`), into 🗑️generated/silhouette/SAV/final/review-<n>.png: review.sh
source "${0:A:h}/env.sh"
F="$GEN/final"; mkdir -p "$F"
all=($=ANIMALS $=VEHICLES)
for a in $all; do
  f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  last="$(python3 -c 'import json,sys; r=json.load(open(sys.argv[1]))["asset"]["representations"]; print([d for d in ("50","100","200","500") if r[d]["status"]=="authored"][-1])' "$f")"
  big=0; [[ "$a" == city-bus || "$a" == tram ]] && big=1
  DENSITY=$([[ $big == 1 ]] && echo 250 || echo 500) "$W/sheets.sh" "$a" - 50 > /dev/null 2>&1
  DENSITY=$([[ $big == 1 ]] && echo 600 || echo 1200) "$W/sheets.sh" "$a" - "$last" > /dev/null 2>&1
  echo "$a 50 $last"
done > "$F/sheets.txt"
i=0; for group in "dairy-cow domestic-cat goat" "hedgehog herring-gull horse" "labrador-dog mallard-duck pig" "pigeon red-fox red-squirrel" "roe-deer sheep cargo-bike" "city-bicycle city-bus city-car" "delivery-van electric-kick-scooter motor-scooter" "tram"; do
  i=$((i+1)); args=()
  for a in $=group; do last="$(grep "^$a " "$F/sheets.txt" | cut -d' ' -f3)"; args+=("$a 1:50 (before above, after below)=$GEN/look/$a.50.png" "$a 1:$last=$GEN/look/$a.$last.png"); done
  (cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/stack.ts" "$F/review-$i.png" 760 $args) > /dev/null 2>&1
done
