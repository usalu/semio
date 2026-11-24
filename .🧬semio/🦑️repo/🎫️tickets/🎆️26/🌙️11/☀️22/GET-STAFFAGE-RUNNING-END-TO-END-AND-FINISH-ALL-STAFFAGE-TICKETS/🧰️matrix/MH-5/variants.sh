#!/bin/zsh
# 🧫️ Several trial variants of one asset, measured together: variants.sh <id> <scales> <name>=<ops json text>… writes each ops text to $GEN/try/<id>.<name>.ops.json, applies it to the catalog file into $GEN/try/<id>.<name>.json (the catalog is not written), prints the lines of the fit that changed or stayed OVER and the model gate of the scales, then the silhouette guard of the backup and of every variant in one process (sil.ts).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; shift 2; files=("$GEN/backup/$a.json"); mkdir -p "$GEN/try"
cd "$T"
for pair in "$@"; do
  name="${pair%%=*}"; print -r -- "${pair#*=}" > "$GEN/try/$a.$name.ops.json"
  echo "== $name"
  "$B" "$W/fit.ts" "$a" --ops "$GEN/try/$a.$name.ops.json" --out "$GEN/try/$a.$name.json" | grep -vE "^written|: unchanged:" | cut -c1-${COLS:-260}
  "$B" "$W/mgate.ts" "$GEN/try/$a.$name.json" --scales "$scales" | grep -E "HARD" | cut -c1-200
  files+=("$GEN/try/$a.$name.json")
done
cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1500 "$B" "$W/sil.ts" --scales "$scales" $files
