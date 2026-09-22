#!/bin/zsh
# 🚨️ Slice CA1 — ONE fleet-wasm-mutex hold, one `describe` per plugin whose SOURCE this slice gave a
# missing `action_destructive` / `action_audience` declaration (📓️ca1-capability-audit-zero.md §2).
# The committed descriptor is the only thing `semio-os-mcp audit` reads, so a source fix reaches the
# gate only through its own producer — nothing here hand-edits a 🔣️.json or 🛂️.descriptor.semio.
# Cheapest owner first so a cut leaves the most rows landed; 🀄️wfc and 🧩️puzzle are CE3's describes.
# Usage: zsh 📜️mutex-ordered.sh 20260922110400 ca1 -- zsh 📜️ca1-describe-batch.sh
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/ca1-describe-ledger.txt"
mkdir -p "$GEN"
cd /Users/ueli/Documents/semio || exit 1
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
OWNERS=("➗️mathematical" "📖️playbook" "📏️layout" "🎬️sequence" "💠️lowpoly" "🏛️architect" "🔱️trinity" "🔋️energy" "🌿️vcs" "🌍️gis" "📕️norm" "🎪️demonstrator")
[ $# -gt 0 ] && OWNERS=("$@")
for OWNER in $OWNERS; do
  RS="✏️s/🔌️plugins/$OWNER/📦️packages/🦀️rust"
  JSON="✏️s/🔌️plugins/$OWNER/🔣️.json"
  PACK="✏️s/🔌️plugins/$OWNER/🛂️.descriptor.semio"
  if [ ! -d "$RS" ]; then printf '%s\tNO_RUST_PACKAGE\t%s\n' "$OWNER" "$(date -Iseconds)" >> "$LEDGER"; continue; fi
  BEFORE_JSON=$(stat -f %z "$JSON" 2>/dev/null || echo 0)
  BEFORE_PACK=$(stat -f %z "$PACK" 2>/dev/null || echo 0)
  START=$(date +%s)
  ( cd "$RS" && bun ./📜️script.ts describe ) > "$GEN/ca1-describe-$OWNER.txt" 2>&1
  RC=$?
  ELAPSED=$(( $(date +%s) - START ))
  AFTER_JSON=$(stat -f %z "$JSON" 2>/dev/null || echo 0)
  AFTER_PACK=$(stat -f %z "$PACK" 2>/dev/null || echo 0)
  printf '%s\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\t%s\n' \
    "$OWNER" "$RC" "$ELAPSED" "$BEFORE_JSON" "$AFTER_JSON" "$BEFORE_PACK" "$AFTER_PACK" "$(date -Iseconds)" >> "$LEDGER"
  echo "=== $OWNER exit $RC after ${ELAPSED}s ===" >> "$GEN/ca1-describe-$OWNER.txt"
  tail -3 "$GEN/ca1-describe-$OWNER.txt"
done
echo "--- ca1 ledger ---"; cat "$LEDGER"
