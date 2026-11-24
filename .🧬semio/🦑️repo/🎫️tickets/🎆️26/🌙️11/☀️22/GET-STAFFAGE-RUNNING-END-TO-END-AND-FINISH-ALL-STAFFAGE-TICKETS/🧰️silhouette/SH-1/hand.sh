#!/bin/zsh
# ✋️ Named repair steps by hand on one person and scale (HAND of auto.ts, read from stdin, one step per line), no greedy pass after them unless GREEDY=1; the ops go to `ops/<slug>.json` and are applied to the catalog file. hand.sh <slug> <scale> < steps
source "${0:A:h}/env.sh"
export WHOLE=1
a="$1"; s="$2"; ops="$W/ops/$a.json"
mkdir -p "$GEN/hand" "$GEN/backup"
[[ -f "$GEN/backup/$a.json" ]] || cp "$CAT/$a/🔣️.json" "$GEN/backup/$a.json"
steps="$(cat)"
cd "$S/📦️packages/🟦️typescript"
if [[ -n "$GREEDY" ]]; then HAND="$steps" MAXSEC=${MAXSEC:-240} timeout 900 "$B" "$W/auto.ts" "$GEN/backup/$a.json" "$s" "$ops" 60 "$ops" > "$GEN/hand/$a-$s.log" 2>&1
else HAND="$steps" PRUNE=0 PARAM=0 POLISH=0 timeout 900 "$B" "$W/auto.ts" "$GEN/backup/$a.json" "$s" "$ops" 0 "$ops" > "$GEN/hand/$a-$s.log" 2>&1; fi
grep -E "hand |taken back|issues |final |rror" "$GEN/hand/$a-$s.log" | cut -c9-260
[[ -n "$NOAPPLY" ]] || "$W/keep.sh" "$a"
