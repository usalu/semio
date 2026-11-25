#!/bin/zsh
# 🥖️ A loft trial: lt.sh <id> <scales> <name>=<lofts.json>… applies ops/<id>.json and the loft entries to the backup into $GEN/try/<id>.<name>.json and prints the loft lines, the model gate, the silhouette guard of the backup and every variant (sil.ts) and, with GATE=1, the hard findings of the real gate at the scales.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; shift 2; files=("$GEN/backup/$a.json")
for pair in "$@"; do
  name="${pair%%=*}"; o="$GEN/try/$a.$name.ops.json"; cp "${OPS:-$W/ops/$a.json}" "$o"; cp "${pair#*=}" "${o%.json}.lofts.json"; out="$GEN/try/$a.$name.json"
  echo "== $name"; "$W/apply.sh" "$GEN/backup/$a.json" "$o" "$out" | grep -E "REFUSED|OVER|loft|changed" | cut -c1-220
  (cd "$T" && "$W/b.sh" "$W/mgate.ts" "$out" --scales "$scales" | grep -E "HARD|unmodelled" | cut -c1-220)
  [[ -n "$GATE" ]] && (cd "$T" && timeout 2400 "$B" "$M/gate.ts" "$out" --scales "$scales" | grep -A4 -E " HARD |hard findings" | cut -c1-250)
  files+=("$out")
done
[[ "$SIL" == 0 ]] || (cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1500 "$B" "$W/sil.ts" --scales "$scales" $files)
