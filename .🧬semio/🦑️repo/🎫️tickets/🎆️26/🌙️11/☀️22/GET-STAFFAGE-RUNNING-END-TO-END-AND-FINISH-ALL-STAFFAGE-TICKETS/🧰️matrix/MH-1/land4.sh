#!/bin/zsh
# 🛬️ Landing of measured trials after the catalog-wide tag change of 2026-11-25 19:09 (the backups carry the old tags, so `take.sh` no longer applies): per id `carry.py` writes the trial $GEN/try/<id>.json into the catalog with the members other workers changed kept, `fit.ts` replays ops/<id>.json without writing (it must change nothing), then the light proof of proof.sh (STEPS, default "gate scan measure"), the check CLI and the matrix run on the written file; the file before is kept in $GEN/take/<id>.was4.json and put back when the replay still changes something: land4.sh <id>…   (run it inside a grant: a job of lane.sh; NOBIG=1 leaves check and matrix out)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/take"
for a in "$@"; do
  f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; cp "$f" "$GEN/take/$a.was4.json"
  python3 "$W/carry.py" "$f" "$GEN/backup/$a.json" "$GEN/try/$a.json" --over --write || { echo "$a not landed"; continue }
  replay="$(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" 2>&1 | tail -1)"; echo "$a replay: $replay"
  [[ "$replay" == *" 0 changed something"* ]] || { echo "$a: the replay still changes the file, put back"; cp "$GEN/take/$a.was4.json" "$f"; continue }
  STEPS="${STEPS:-gate scan measure}" INSIDE=light "$W/proof.sh" "$a"
  [[ -n "$NOBIG" ]] || { INSIDE=check "$W/proof.sh" "$a"; INSIDE=matrix "$W/proof.sh" "$a" }
done
