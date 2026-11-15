#!/usr/bin/env bash
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
for pair in "$@"; do
  chapter="${pair%%:*}"
  scene="${pair##*:}"
  bash "$TK/demand-b/direct.sh" "$chapter" "$scene"
done
echo done > "$TK/🗑️generated/demand-b-out/direct/chain.done"
