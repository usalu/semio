#!/bin/zsh
# 🪐️ Slice S8 — S4's narrow `🪐️space` re-stage recipe on S8's own private target dir and captures.
# One component build, one materialize, one activation — no cold boot, every wasm32 step through the
# fleet mutex (preamble rule 27).
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
MUTEX="$TICKET/📜️wasm-build-mutex.sh"
OUT="$TICKET/🗑️generated"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-s8"

echo "[s8-restage] step 1 component dev  $(date '+%F %T')"
zsh "$MUTEX" s8 -- bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts" \
  native component dev --manifest "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml" > "$OUT/s8-space-component.txt" 2>&1
echo "[s8-restage] step 1 rc=$? $(date '+%F %T')"

echo "[s8-restage] step 2 materialize  $(date '+%F %T')"
zsh "$MUTEX" s8 -- bun nx run @semio-tech/space-plugin:materialize-dev > "$OUT/s8-materialize.txt" 2>&1
echo "[s8-restage] step 2 rc=$? $(date '+%F %T')"

echo "[s8-restage] step 3 activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" \
  && zsh "/Users/ueli/Documents/semio/$MUTEX" s8 -- bun ./📜️script.ts activate s react dev ) > "$OUT/s8-activate.txt" 2>&1
echo "[s8-restage] step 3 rc=$? $(date '+%F %T')"
echo "[s8-restage] DONE $(date '+%F %T')"
