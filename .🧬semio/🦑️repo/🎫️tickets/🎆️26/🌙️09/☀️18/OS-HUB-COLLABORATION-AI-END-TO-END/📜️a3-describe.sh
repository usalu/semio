#!/usr/bin/env zsh
# 🖨️ Slice A3 — regenerate a shipped plugin descriptor through the plugin's OWN `describe` verb
# (`bun ./📜️script.ts describe` → `describePluginComponent`: wasm32-wasip2 `wasm-dev` component
# build, jco core extraction, then `🛂️.descriptor.semio` + `🔣️.json` at the plugin's owner root).
# Never hand-edits a generated file. Records bytes before/after and wall time per target.
# Usage: 📜️a3-describe.sh <owner-dir-relative-to-✏️s/🔌️plugins> ...
#   e.g. 📜️a3-describe.sh "➗️mathematical" "📐️cad/🧩️extensions/📐️spatial-shape"
# Uses the SHARED artifact dir on purpose: the wasm-dev artifacts for these crates are already warm
# there (M5a's 11:51 note describe compiled in 16 s through it); a private dir would pay a cold
# component build per plugin.
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/a3-describe-ledger.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

for owner in "$@"; do
  base="$ROOT/✏️s/🔌️plugins/$owner"
  dir="$base/📦️packages/🦀️rust"
  slug="$(echo "$owner" | tr '/' '_')"
  LOG="$GEN/a3-describe-$slug.txt"
  if [ ! -d "$dir" ]; then echo "$owner	NO_RUST_PACKAGE" >> "$LEDGER"; continue; fi
  before_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
  before_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
  start=$(date +%s)
  echo "=== describe $owner at $(date -Iseconds) (json=$before_json pack=$before_pack) ===" > "$LOG"
  ( cd "$dir" && zsh "$TICKET/📜️wasm-build-mutex.sh" a3 -- bun ./📜️script.ts describe ) >> "$LOG" 2>&1
  rc=$?
  end=$(date +%s)
  echo "=== exit $rc after $((end-start))s ===" >> "$LOG"
  after_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
  after_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
  printf '%s\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\t%s\n' \
    "$owner" "$rc" "$((end-start))" "$before_json" "$after_json" "$before_pack" "$after_pack" \
    "$(date -Iseconds)" >> "$LEDGER"
  tail -3 "$LOG"
done
printf '\n--- ledger ---\n'
cat "$LEDGER"
