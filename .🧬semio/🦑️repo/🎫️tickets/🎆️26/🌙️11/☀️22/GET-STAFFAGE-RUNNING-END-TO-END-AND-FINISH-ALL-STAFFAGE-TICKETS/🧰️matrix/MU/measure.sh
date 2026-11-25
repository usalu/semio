#!/bin/zsh
# 📐️ The silhouette guard, validate and three-views measure (`../MR/measure.ts`) of the untouched document ($GEN/backup or the catalog) and of the trial ($GEN/try/<id>.json) of an asset, one process per scale so that the memory stays small; prints the lines that differ and every line with a missed gate (`!`): measure.sh <id>   (run it inside a grant)
source "${0:h}/env.sh"
a="$1"; base="$GEN/backup/$a.json"; [[ -f "$base" ]] || base="$(asset "$a")"; cd "$P"; mkdir -p "$GEN/measure"
for k in 50 100 200 500; do
  timeout 900 "$B" "$M/measure.ts" "$base" --scales $k > "$GEN/measure/$a.$k.was" 2>&1
  timeout 900 "$B" "$M/measure.ts" "$GEN/try/$a.json" --scales $k > "$GEN/measure/$a.$k.now" 2>&1
done
cat "$GEN"/measure/$a.{50,100,200,500}.was > "$GEN/measure/$a.was"; cat "$GEN"/measure/$a.{50,100,200,500}.now > "$GEN/measure/$a.now"
cmp -s "$GEN/measure/$a.was" "$GEN/measure/$a.now" && echo "measure $a identical" || diff "$GEN/measure/$a.was" "$GEN/measure/$a.now" | cut -c1-260
