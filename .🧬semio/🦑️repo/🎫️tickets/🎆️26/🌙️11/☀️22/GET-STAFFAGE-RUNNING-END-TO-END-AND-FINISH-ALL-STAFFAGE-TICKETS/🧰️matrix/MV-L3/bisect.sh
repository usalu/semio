#!/bin/zsh
# 🪓️ Which masses carry the guard of a scale as lofts, inside one grant: `bisect.sh <id> <scale> <bone>…` applies, per bone, the saved lofts of that one bone at that scale (`pick.py`, `../MV-L1/loft.ts --ops`) to the backup and prints the guard line of the scale (`../MR/measure.ts --scales`); a bone `-a,b` stands for every bone but those, `*` for all.
source "${0:A:h}/env.sh"
a="$1"; s="$2"; shift 2; mkdir -p "$GEN/bisect"
for b in "$@"; do
  python3 "$W/pick.py" "$W/loft/$a.json" "$GEN/bisect/ops.json" "$s:$b" > /dev/null
  (cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MV-L1/loft.ts" "$GEN/backup/$a.json" --ops "$GEN/bisect/ops.json" --out "$GEN/bisect/copy.json" > "$GEN/bisect/apply.log" 2>&1) || { echo "$b: apply failed $(tail -1 "$GEN/bisect/apply.log" | cut -c1-200)"; continue; }
  echo "$b | $(cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/bisect/copy.json" --scales "$s" 2>&1 | grep bbox | sed 's/.*bbox [^ ]* [^ ]* [^ ]* *//' | cut -c1-200)"
done
