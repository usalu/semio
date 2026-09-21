#!/usr/bin/env zsh
# 🖨️ Slice JB1 — ONE fleet-wasm-mutex hold, several plugin `describe` runs in order.
# Wraps `📜️a3-describe.sh`'s per-owner recipe but takes the mutex ONCE for the whole batch
# (preamble rule 27(a) + rule 17: one foreground call, no re-queueing between owners).
# Usage: 📜️jb1-describe-batch.sh <owner-dir-relative-to-✏️s/🔌️plugins> ...
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
mkdir -p "$GEN"
LEDGER="$GEN/jb1-describe-ledger.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

for owner in "$@"; do
  base="$ROOT/✏️s/🔌️plugins/$owner"
  dir="$base/📦️packages/🦀️rust"
  slug="$(echo "$owner" | tr '/' '_')"
  LOG="$GEN/jb1-describe-$slug.txt"
  if [ ! -d "$dir" ]; then echo "$owner	NO_RUST_PACKAGE" >> "$LEDGER"; continue; fi
  before_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
  before_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
  start=$(date +%s)
  echo "=== describe $owner at $(date -Iseconds) (json=$before_json pack=$before_pack) ===" > "$LOG"
  ( cd "$dir" && bun ./📜️script.ts describe ) >> "$LOG" 2>&1
  rc=$?
  end=$(date +%s)
  echo "=== exit $rc after $((end-start))s ===" >> "$LOG"
  after_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
  after_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
  printf '%s\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\t%s\n' \
    "$owner" "$rc" "$((end-start))" "$before_json" "$after_json" "$before_pack" "$after_pack" \
    "$(date -Iseconds)" >> "$LEDGER"
  tail -4 "$LOG"
done
printf '\n--- jb1 ledger ---\n'
cat "$LEDGER"
