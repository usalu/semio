#!/bin/zsh
# 📏️ The measurement before the repair, one asset after the other: the proposals of `suggest.ts` (hard tier and `--all`) into $GEN/suggest and the full gate into $GEN/gate/before-<id>.log: baseline.sh <id>…
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/suggest"
for a in "$@"; do
  "$W/q.sh" 1000 1800 "$T" "$B" "$M/suggest.ts" "$a" > "$GEN/suggest/$a.json" 2>&1; "$W/q.sh" 1000 1800 "$T" "$B" "$M/suggest.ts" "$a" --all > "$GEN/suggest/$a.all.json" 2>&1
  "${0:A:h}/g.sh" "$a" "before-$a"
done
