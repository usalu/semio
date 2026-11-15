#!/usr/bin/env bash
cd /c/git/semio || exit 1
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/demand-a-out"
LANG_CODE="${LANG_CODE:-de}"
for chapter in "$@"; do
  echo "=== render $chapter $(date +%T)"
  repo tools exec bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language "$LANG_CODE" --video energy_balance --chapter "$chapter" --profile preview --audience speaker --jobs 2 --no-play --output-root "$OUT" > "$OUT/render-$chapter.log" 2>&1
  echo "exit $? $(date +%T)"
  tail -5 "$OUT/render-$chapter.log"
done
