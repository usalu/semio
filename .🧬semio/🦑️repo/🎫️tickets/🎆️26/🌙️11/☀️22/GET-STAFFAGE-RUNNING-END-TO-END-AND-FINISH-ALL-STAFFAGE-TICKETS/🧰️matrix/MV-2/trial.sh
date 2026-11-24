#!/bin/zsh
# 🧫️ The search run of one plant on a copy: the catalog file is copied to `$GEN/try/<id>.json`, the ops `ops/<id>.json` are applied when they exist, then `carry.ts` (CARRY=0 skips it), the anchor search on that copy (new anchor ops are added to `ops/<id>.json` and applied to the copy), then the gate over every subject. Nothing in the catalog is written. trial.sh <id> [carry flags…]
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
a="$1"; shift; mkdir -p "$GEN/try" "$GEN/log" "$W/ops"
cd "$T"
src="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
cp "$src" "$GEN/try/$a.json"
[[ -f "$W/ops/$a.json" ]] && "$B" "$W/apply.ts" "$a" --in "$GEN/try/$a.json" --out "$GEN/try/$a.json"
if [[ "$CARRY" != 0 ]]; then
  timeout 3000 "$B" "$W/carry.ts" "$GEN/try/$a.json" --out "$GEN/try/$a.json" "$@" > "$GEN/log/$a-carry.log" 2>&1; echo "carry exit $? $(grep -c 'STILL BEYOND' "$GEN/log/$a-carry.log") beyond; $(grep 'plates with' "$GEN/log/$a-carry.log")"
fi
[[ -f "$W/ops/$a.json" ]] && "$B" "$W/apply.ts" "$a" --in "$GEN/try/$a.json" --out "$GEN/try/$a.json"
timeout 3000 "$B" "$W/anchors.ts" "$GEN/try/$a.json" --ops "$W/ops/$a.json" ${=ANCHOR_FLAGS} > "$GEN/log/$a-anchors.log" 2>&1; echo "anchors exit $?"; cut -c1-330 "$GEN/log/$a-anchors.log"
[[ -f "$W/ops/$a.json" ]] && "$B" "$W/apply.ts" "$a" --in "$GEN/try/$a.json" --out "$GEN/try/$a.json"
timeout 1800 "$B" "$M/gate.ts" "$GEN/try/$a.json" > "$GEN/gate/$a.try.log" 2>&1; echo "gate exit $?"
grep -E "HARD|hard findings|^    " "$GEN/gate/$a.try.log" | cut -c1-260 | awk '/^    /{n++; if(n<=6)print; next}{n=0;print}'
