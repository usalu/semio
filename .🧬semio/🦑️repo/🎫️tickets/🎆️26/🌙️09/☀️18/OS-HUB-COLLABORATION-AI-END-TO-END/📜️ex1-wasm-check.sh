#!/bin/zsh
# 🧩 EX1 — `cargo check --target wasm32-wasip2` over the owners whose ABI `__semio_owned_core_exports!`
# changes: two extensions (single-argument `extension_exports!` arm) and one plugin (regression side).
# Runs through the fleet wasm build mutex (preamble rule 27), one cargo at a time, in this order.
ROOT="/Users/ueli/Documents/semio"
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="$TICKET/🗑️generated/ex1-wasm-check.txt"
crates=("${@:-semio-s-plugin-cad-spatial-shape semio-s-plugin-imperative-text semio-s-plugin-mathematical}")
cd "$ROOT" || exit 9
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
{
  echo "=== ex1 wasm32-wasip2 check queued $(date '+%H:%M:%S')"
  zsh "$TICKET/📜️wasm-build-mutex.sh" ex1 -- zsh -c '
    for crate in '"$crates"'; do
      echo "--- $crate start $(date "+%H:%M:%S")"
      cargo check -p "$crate" --target wasm32-wasip2 --profile wasm-dev 2>&1
      echo "--- $crate rc=$? end $(date "+%H:%M:%S")"
    done
  '
  echo "=== ex1 wasm32-wasip2 check DONE $(date '+%H:%M:%S')"
} > "$OUT" 2>&1
