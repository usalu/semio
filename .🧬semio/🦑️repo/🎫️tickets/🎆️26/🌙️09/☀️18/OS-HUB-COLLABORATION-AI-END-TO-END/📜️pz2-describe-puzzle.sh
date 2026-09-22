#!/usr/bin/env zsh
# 🖨️ Slice PZ2 — takes the fleet wasm mutex at PZ2's fixed queue position and describes 🧩️puzzle.
# Re-queues ONCE when the hold died in under 120 s, which is the signature of a peer's in-flight
# compile break rather than of a real describe failure (PZ1 lost a 98-minute hold to exactly that,
# `📓️pz1-catalog-zero-diagnostics.md` §2.3).
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/pz2-describe-ledger.txt"
STAMP=20260922110500
for attempt in 1 2; do
  printf '%s\tqueued attempt %s at %s\n' "🧩️puzzle" "$attempt" "$(date -Iseconds)" >> "$LEDGER"
  zsh "$TICKET/📜️mutex-ordered.sh" "$STAMP" pz2 -- zsh "$TICKET/📜️pz2-describe-once.sh" "$attempt"
  read -r elapsed rc < "$GEN/pz2-describe-last-attempt.txt"
  [ "$rc" = "0" ] && break
  [ "$elapsed" -ge 120 ] && break
  printf 'attempt %s died in %ss (rc=%s) — shared tree was red, re-queueing\n' "$attempt" "$elapsed" "$rc" >> "$LEDGER"
done
printf '\n--- pz2 ledger ---\n'
cat "$LEDGER"
