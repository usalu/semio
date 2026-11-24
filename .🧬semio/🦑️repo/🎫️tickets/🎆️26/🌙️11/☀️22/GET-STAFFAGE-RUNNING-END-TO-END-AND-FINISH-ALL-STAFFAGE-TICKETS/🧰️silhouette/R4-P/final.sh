#!/bin/zsh
# 🏁️ Ships the four plants from their state before this round and proves them, one process after the other: per plant `make/<id>.sh` on the backup into a work file, every silhouette check of that file, the copy to the catalog (only when every check holds), a second `make` on the catalog file (it must change nothing: compared byte by byte), then `prove.sh` and the sheets of every authored scale. final.sh <id>…
source "${0:A:h}/env.sh"; mkdir -p "$GEN/final"
for a in "$@"; do
  f="$GEN/final/$a.json"
  "$W/make/$a.sh" --in "$GEN/backup/$a.json" --out "$f" | tail -4 | cut -c1-90
  "$W/t.sh" measure.ts "$f" | tee "$GEN/final/$a-measure.txt" | cut -c1-300
  grep -q "^$a: OK" "$GEN/final/$a-measure.txt" || { echo "$a NOT SHIPPED"; continue; }
  cp "$f" "$CAT/$a/🔣️.json"
  "$W/make/$a.sh" > /dev/null; cmp "$f" "$CAT/$a/🔣️.json" && echo "$a | make on the catalog file changes nothing (idempotent), the catalog file is make(state before)" | tee -a "$GEN/proof.txt"
  "$W/prove.sh" "$a" | cut -c1-260
  for s in 50 100 200 500; do "$W/look.sh" final "$a" $s 2>&1 | sed 's|.*/||'; done
done
