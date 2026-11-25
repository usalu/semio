#!/bin/zsh
# 📐️ Silhouette guard, validateAsset and three-views sweep (MR/measure.ts) of the given asset ids, one after the other, then the product check CLI of each; logs in $GEN/measure/<id>.<TAG>.log: TAG=before SRC=backup measure-all.sh <id>…   (SRC names a folder of $GEN holding <id>.json; without it the catalog file is measured and checked)
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
B="${0:A:h}/qbun"
mkdir -p "$GEN/measure"
tag="${TAG:-now}"
for a in "$@"; do
  src="$a"
  [[ -n "$SRC" ]] && src="$GEN/$SRC/$a.json"
  (cd "$S/📦️packages/🟦️typescript" && QMEM=2500 QTIME=900 "$B" "$M/measure.ts" "$src") > "$GEN/measure/$a.$tag.log" 2>&1
  echo "$a measure exit $? $(grep -E 'silhouette:|validate:|sweep:' "$GEN/measure/$a.$tag.log" | sed "s/^$a //" | tr '\n' ';')"
  if [[ -z "$SRC" ]]; then
    (cd "$S" && QMEM=5000 QTIME=900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/measure/$a.$tag.check.log" 2>&1
    echo "$a check exit $? $(grep -m1 -E '^(✓|✗)' "$GEN/measure/$a.$tag.check.log" | cut -c1-160)"
  fi
done
