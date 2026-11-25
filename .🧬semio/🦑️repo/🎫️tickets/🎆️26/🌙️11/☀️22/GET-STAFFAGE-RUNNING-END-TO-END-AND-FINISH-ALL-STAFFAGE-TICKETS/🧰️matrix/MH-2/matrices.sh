#!/bin/zsh
# 📊️ `T/📊️matrix.ts` of several assets under ONE grant of the memory queue, one after the other (called through q.sh and guard.sh): matrices.sh <id>…; per asset the log $GEN/proof/<id>-matrix.log, the catalog file's md5 before and after, and the cells by status in $GEN/proof/matrix-summary.txt.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; cd "$T"
for a in "$@"; do f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; s=$SECONDS; m="$(md5 -q "$f")"; rm -rf "$GEN/matrix-$a"
  "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$GEN/proof/$a-matrix.log" 2>&1; x=$?
  { echo "=== C $a exit $x, $((SECONDS-s)) s, ended $(date +%H:%M), md5 ${m[1,8]} / ${$(md5 -q "$f")[1,8]}: $(tail -1 "$GEN/proof/$a-matrix.log")"; python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | cut -c1-300; } >> "$GEN/proof/matrix-summary.txt"
done
