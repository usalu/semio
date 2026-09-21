#!/bin/zsh
# 🪐️ Slice S9 — S4/S8's narrow `🪐️space` re-stage recipe. Reuses S8's target dir on purpose: disk is
# tight (preamble session note) and a second wasm target tree costs tens of GiB for nothing.
# One component build, one materialize, one activation — every wasm32 step through the fleet mutex.
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
MUTEX="$TICKET/📜️wasm-build-mutex.sh"
OUT="$TICKET/🗑️generated"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-s8"

echo "[s9-restage] step 1 component dev  $(date '+%F %T')"
zsh "$MUTEX" s9 -- bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts" \
  native component dev --manifest "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml" > "$OUT/s9-space-component.txt" 2>&1
echo "[s9-restage] step 1 rc=$? $(date '+%F %T')"

echo "[s9-restage] step 2 materialize  $(date '+%F %T')"
zsh "$MUTEX" s9 -- bun nx run @semio-tech/space-plugin:materialize-dev > "$OUT/s9-materialize.txt" 2>&1
echo "[s9-restage] step 2 rc=$? $(date '+%F %T')"

echo "[s9-restage] step 3 activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" \
  && zsh "/Users/ueli/Documents/semio/$MUTEX" s9 -- bun ./📜️script.ts activate s react dev ) > "$OUT/s9-activate.txt" 2>&1
echo "[s9-restage] step 3 rc=$? $(date '+%F %T')"
echo "[s9-restage] DONE $(date '+%F %T')"
