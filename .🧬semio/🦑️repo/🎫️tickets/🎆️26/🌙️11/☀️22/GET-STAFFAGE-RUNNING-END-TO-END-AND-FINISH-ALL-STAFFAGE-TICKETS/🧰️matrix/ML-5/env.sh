# 🧭️ Paths of the multiview fixer ML-5 (source it): the MR paths with W and GEN pointing at this batch, H the folder of MH-4 whose chain (fit.ts, lofta.ts, gate.ts) this round continues.
export BATCH=ML-5
source "${0:A:h}/../MR/env.sh"
export H="$U/🧰️matrix/MH-4"
export P="$S/📦️packages/🟦️typescript"
export C="$S/🗂️catalog/👤️humans"
export IDS=(person-stretching person-with-white-cane person-with-guide-dog)
mkdir -p "$GEN/try" "$GEN/base" "$GEN/proof" "$GEN/vt" "$GEN/sheets"
cap() {
  local s="$1" dir="$2"; shift 2
  (cd "$dir" && /usr/bin/time -l -o "$GEN/rss.$$" timeout "$s" "$@"); local code=$?
  echo "$(date +%H:%M:%S) $(( $(awk '/maximum resident set size/ { print $1 }' "$GEN/rss.$$") / 1048576 )) MB peak: ${2:t} ${3:t} ${4:t}" >> "$GEN/rss.log"; rm -f "$GEN/rss.$$"
  return $code
}
