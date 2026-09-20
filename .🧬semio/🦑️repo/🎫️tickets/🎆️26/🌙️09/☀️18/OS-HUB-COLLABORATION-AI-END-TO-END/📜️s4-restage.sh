#!/bin/zsh
# 🪐️ Slice S4 — the narrow re-stage of the fixed `🪐️space` guest into the already-served `s` output
# (S2 §3.4's recipe, S3 §3.4's private-target-dir cure, preamble rule 27's fleet wasm mutex around
# EVERY wasm32 step). One component build, one materialize, one activation — no cold boot.
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
MUTEX="$TICKET/📜️wasm-build-mutex.sh"
OUT="$TICKET/🗑️generated"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-s4"

echo "[s4-restage] step 1 component dev  $(date '+%F %T')"
zsh "$MUTEX" s4 -- bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts" \
  native component dev --manifest "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml" > "$OUT/s4-space-component.txt" 2>&1
echo "[s4-restage] step 1 rc=$? $(date '+%F %T')"

echo "[s4-restage] step 2 materialize  $(date '+%F %T')"
zsh "$MUTEX" s4 -- bun nx run @semio-tech/space-plugin:materialize-dev > "$OUT/s4-materialize.txt" 2>&1
echo "[s4-restage] step 2 rc=$? $(date '+%F %T')"

echo "[s4-restage] step 3 activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" \
  && zsh "/Users/ueli/Documents/semio/$MUTEX" s4 -- bun ./📜️script.ts activate s react dev ) > "$OUT/s4-activate.txt" 2>&1
echo "[s4-restage] step 3 rc=$? $(date '+%F %T')"
echo "[s4-restage] DONE $(date '+%F %T')"
