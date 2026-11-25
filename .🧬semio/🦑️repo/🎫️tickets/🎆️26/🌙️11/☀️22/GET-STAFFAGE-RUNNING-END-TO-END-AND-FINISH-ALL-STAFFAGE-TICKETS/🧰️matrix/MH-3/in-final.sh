#!/bin/zsh
# 🏁️ The two heavy proofs of written assets inside ONE grant of 1200 MB (start it with qsh; queue rule of 2026-11-25 09:20), one process after the other: per asset the product check CLI (`--asset <id>`) and the matrix confirmation `T/📊️matrix.ts --language en --out $GEN/matrix-<id> --jobs 1 --fresh --assets <id>`, each under `/usr/bin/time -l` so that the peak memory of the process tree is on record, then the cells of the asset by status and failing check; the md5 of the catalog file is printed before and after, so a proof is tied to the file it measured: QMEM=1200 QTIME=<s> qsh in-final.sh <id>…   (logs in $GEN/proof/<id>-check.log and <id>-matrix.log)
[[ -n "$MH3_GRANT" ]] || { echo "in-final.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
peak() { awk '/maximum resident set size/ { printf "%d MB", $1 / 1048576 }' "$1"; }
for a in "$@"; do
  file="$S/🗂️catalog/👤️humans/$a/🔣️.json"; before="$(md5 -q "$file")"
  start=$(date +%s)
  (cd "$S" && /usr/bin/time -l timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1
  echo "$a check exit $? ($(( $(date +%s) - start )) s, peak $(peak "$d/$a-check.log")): $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-200)"
  start=$(date +%s)
  (cd "$T" && /usr/bin/time -l timeout ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1
  echo "$a matrix exit $? ($(( $(date +%s) - start )) s, peak $(peak "$d/$a-matrix.log")): $(grep -v -E '^ +[0-9]+ +[a-z ]+$|^$' "$d/$a-matrix.log" | grep -v 'maximum resident\|real \|user \|sys ' | tail -1 | cut -c1-200)"
  python3 "$here/cells-by-asset.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300
  echo "$a file md5 $before -> $(md5 -q "$file") $(date '+%H:%M')"
done
