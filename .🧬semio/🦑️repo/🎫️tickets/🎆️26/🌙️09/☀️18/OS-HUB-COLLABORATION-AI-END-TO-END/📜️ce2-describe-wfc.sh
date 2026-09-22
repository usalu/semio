#!/bin/zsh
# 🀄️ Re-describes 🀄️wfc (and then 🧩️puzzle) through the fleet wasm mutex, after slice CE2 made the
# describe deadline a no-fuel-progress bound instead of a total-wall one. Both plugins previously
# died `epoch deadline exceeded` while still progressing normally (📓️ce2-mcp-gates-green.md §5).
# One mutex acquisition for the whole batch, a ledger row per owner, detached and survivable.
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/ce2-describe-ledger.txt"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864
for OWNER in "🀄️wfc" "🧩️puzzle"; do
  RS="✏️s/🔌️plugins/$OWNER/📦️packages/🦀️rust"
  JSON="✏️s/🔌️plugins/$OWNER/🔣️.json"
  PACK="✏️s/🔌️plugins/$OWNER/🛂️.descriptor.semio"
  BEFORE_JSON=$(stat -f %z "$JSON" 2>/dev/null || echo 0)
  BEFORE_PACK=$(stat -f %z "$PACK" 2>/dev/null || echo 0)
  START=$(date +%s)
  ( cd "$RS" && bun ./📜️script.ts describe ) > "$GEN/ce2-describe-$OWNER.txt" 2>&1
  RC=$?
  ELAPSED=$(( $(date +%s) - START ))
  AFTER_JSON=$(stat -f %z "$JSON" 2>/dev/null || echo 0)
  AFTER_PACK=$(stat -f %z "$PACK" 2>/dev/null || echo 0)
  printf "%s\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\t%s\n" \
    "$OWNER" "$RC" "$ELAPSED" "$BEFORE_JSON" "$AFTER_JSON" "$BEFORE_PACK" "$AFTER_PACK" "$(date -Iseconds)" >> "$LEDGER"
  echo "=== $OWNER exit $RC after ${ELAPSED}s ===" >> "$GEN/ce2-describe-$OWNER.txt"
done
echo "--- ce2 ledger ---"; cat "$LEDGER"
