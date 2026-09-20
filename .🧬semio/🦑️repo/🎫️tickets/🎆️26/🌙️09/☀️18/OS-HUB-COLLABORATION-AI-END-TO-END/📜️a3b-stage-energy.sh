#!/bin/zsh
# 🔋️ A3b: build the `energy` plugin component `client-e2e`'s own `capabilities_search` hit 0 now
# resolves to (`energy.s.energy.model@1/*#editor.set-cell`) — the gate's three dispatch rows are red
# only because no `semio_s_plugin_energy.wasm` exists in the SHARED uplift dir the gateway resolves.
# Built straight into the shared dir (the `describe` verb builds the same way), under the fleet wasm
# mutex (preamble rule 27). Caller wraps nothing: this script takes the mutex itself.
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="$TICKET/🗑️generated/a3b-stage-energy.txt"
cd "$ROOT" || exit 9
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
{
  echo "=== a3b energy component build queued $(date '+%H:%M:%S')"
  zsh "$TICKET/📜️wasm-build-mutex.sh" a3b -- cargo build -p semio-s-plugin-energy --target wasm32-wasip2 --profile wasm-dev
  echo "=== rc=$? $(date '+%H:%M:%S')"
  ls -l "$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_energy.wasm" 2>&1
} > "$OUT" 2>&1
