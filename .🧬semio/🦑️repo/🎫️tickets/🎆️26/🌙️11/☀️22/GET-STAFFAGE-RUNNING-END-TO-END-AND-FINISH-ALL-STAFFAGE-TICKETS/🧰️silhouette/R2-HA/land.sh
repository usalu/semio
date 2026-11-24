#!/bin/zsh
# 🛬️ Lands the work ops of a person: copies them to `ops/<slug>.json` of this folder (the repair of this round, run after the ops of batch SH-2) and applies them to the catalog file: the two plainly serialised files of the generator chains through `apply.ts`, the hand-laid-out files through the span-preserving `apply.py`: land.sh <slug>…   (REBUILD=1: from the state before this round, the backup, so ops taken back leave no trace)
source "${0:A:h}/env.sh"
for a in "$@"; do
  [[ -f "$GEN/ops/$a.json" ]] && cp "$GEN/ops/$a.json" "$W/ops/$a.json"
  [[ -n "$REBUILD" ]] && cp "$GEN/backup/$a.json" "$CAT/$a/🔣️.json"
  if [[ "$a" == person-taking-selfie || "$a" == person-with-guide-dog ]]; then (cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/apply.ts" "$a"); else (cd "$W" && python3 apply.py "$a"); fi
done
