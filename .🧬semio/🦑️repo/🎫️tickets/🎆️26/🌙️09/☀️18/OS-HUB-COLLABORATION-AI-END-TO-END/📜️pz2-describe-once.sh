#!/usr/bin/env zsh
# 🖨️ Slice PZ2 — the body of ONE `describe` of 🧩️puzzle, run INSIDE a fleet-wasm-mutex hold by
# `📜️pz2-describe-puzzle.sh`. Writes its own capture and one ledger row.
# Usage: 📜️pz2-describe-once.sh <attempt>
set -u
attempt="${1:-1}"
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/pz2-describe-ledger.txt"
owner="🧩️puzzle"
base="$ROOT/✏️s/🔌️plugins/$owner"
dir="$base/📦️packages/🦀️rust"
log="$GEN/pz2-describe-puzzle-$attempt.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true
before_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
before_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
start=$(date +%s)
echo "=== describe $owner attempt $attempt at $(date -Iseconds) (json=$before_json pack=$before_pack) ===" > "$log"
( cd "$dir" && bun ./📜️script.ts describe ) >> "$log" 2>&1
rc=$?
end=$(date +%s)
echo "=== exit $rc after $((end-start))s ===" >> "$log"
after_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
after_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
printf '%s\tattempt=%s\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\t%s\n' \
  "$owner" "$attempt" "$rc" "$((end-start))" "$before_json" "$after_json" "$before_pack" "$after_pack" "$(date -Iseconds)" >> "$LEDGER"
printf '%s %s\n' "$((end-start))" "$rc" > "$GEN/pz2-describe-last-attempt.txt"
exit $rc
