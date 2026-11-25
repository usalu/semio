#!/bin/zsh
# 🧫️ Tries the repair of assets without touching the catalog, in one queue grant, one process after the other: runs the ops writer `make/<id>.py`, applies `ops/<id>.json` to the catalog document into $GEN/try/<id>.json (apply.ts), then on the trial copies the boxes of every shape (boxes.ts into $GEN/boxes-try), the fixer's gate over every subject ($GEN/log/try-gate-<id>.txt), `validateAsset`, the print check of every band and its range ends, and the silhouette, validate and three-views sweep measure of `../MR/measure.ts`; with BEFORE=1 the print check and the measure also run on the backups (once, kept in $GEN/log/before-*). try.sh <id>…   (run it through q.sh with 2500 MB; ids listed in $GEN/try-more.txt are added to a run that still waits in the queue)
source "${0:h}/env.sh"
mkdir -p "$GEN/try" "$GEN/log"
tries=()
ids=("$@"); [[ -f "$GEN/try-more.txt" ]] && ids+=($(cat "$GEN/try-more.txt")) && rm -f "$GEN/try-more.txt"
for a in $ids; do
  python3 "$W/make/$a.py" > "$GEN/log/try-make-$a.txt" 2>&1 || { echo "$a: the ops writer failed: $(tail -1 "$GEN/log/try-make-$a.txt")"; continue; }
  (cd "$P" && timeout 300 "$B" "$W/apply.ts" "$a" --out "$GEN/try/$a.json") > "$GEN/log/try-apply-$a.txt" 2>&1 || { echo "$a: apply failed: $(tail -3 "$GEN/log/try-apply-$a.txt" | cut -c1-300)"; continue; }
  tries+=("$GEN/try/$a.json")
done
[[ ${#tries} -eq 0 ]] && exit 1
cd "$T"
timeout 1500 "$B" "$W/boxes.ts" "$GEN/boxes-try" $tries > "$GEN/log/try-boxes.txt" 2>&1
for f in $tries; do
  a="${f:t:r}"
  timeout 1500 "$B" "$M/gate.ts" "$f" --soft > "$GEN/log/try-gate-$a.txt" 2>&1
  echo "$a gate: $(grep -c ' HARD ' "$GEN/log/try-gate-$a.txt") hard codes, $(grep '^hard findings' "$GEN/log/try-gate-$a.txt") (before $(python3 "$W/gate-table.py" "$GEN/log/before-gate.txt" "$a"))"
  grep ' HARD ' "$GEN/log/try-gate-$a.txt" | cut -c1-260
done
timeout 900 "$B" "$W/prints.ts" $tries > "$GEN/log/try-prints.txt" 2>&1
(cd "$P" && timeout 900 "$B" "$W/validate.ts" $tries) > "$GEN/log/try-validate.txt" 2>&1
(cd "$P" && timeout 2400 "$B" "$M/measure.ts" $tries) > "$GEN/log/try-measure.txt" 2>&1
if [[ -n "$BEFORE" ]]; then
  backs=(); for f in $tries; do backs+=("$GEN/backup/${f:t}"); done
  timeout 900 "$B" "$W/prints.ts" $backs > "$GEN/log/before-prints.txt" 2>&1
  (cd "$P" && timeout 2400 "$B" "$M/measure.ts" $backs) > "$GEN/log/before-measure.txt" 2>&1
fi
for f in $tries; do
  a="${f:t:r}"
  echo "$a validate: $(grep "^$a" "$GEN/log/try-validate.txt" | cut -f2-) | print: $(grep "^$a" "$GEN/log/try-prints.txt" | cut -f2) (before $(grep "^$a" "$GEN/log/before-prints.txt" 2>/dev/null | cut -f2)) | $(grep "^$a silhouette" "$GEN/log/try-measure.txt") (before $(grep "^$a silhouette" "$GEN/log/before-measure.txt" 2>/dev/null | cut -d: -f2)) | $(grep "^$a three-views" "$GEN/log/try-measure.txt" | cut -c1-120)"
done
