#!/usr/bin/env zsh
# ⏳️ Slice PZ1 — wait for the shared tree to compile again OUTSIDE the fleet wasm mutex, then take
# the mutex once and run the describes. Two peer-owned breakages (the `Mutation::label` →
# `LocalizedLabel` sweep at 04:03, `UiNodeRecord: Clone` at 13:39) each wasted a mutex hold on a
# `cargo build … exited with status 101` four seconds in; holding the fleet lock while polling for
# somebody else's fix would be worse, so the polling happens here and the lock is taken last.
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-pz1"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
cd "$ROOT" || exit 1
deadline=$(( $(date +%s) + 14400 ))
while [ "$(date +%s)" -lt "$deadline" ]; do
  if cargo check -q -p semio-framework-plugin -p semio-framework-ui-contract >/dev/null 2>&1; then
    echo "tree green at $(date -Iseconds)" >> "$GEN/pz1-await.txt"
    exec zsh "$TICKET/📜️wasm-build-mutex.sh" pz1 -- zsh "$TICKET/📜️pz1-describe-batch.sh" "$@"
  fi
  echo "tree red at $(date -Iseconds)" >> "$GEN/pz1-await.txt"
  sleep 120
done
echo "gave up waiting for a green tree at $(date -Iseconds)" >> "$GEN/pz1-await.txt"
