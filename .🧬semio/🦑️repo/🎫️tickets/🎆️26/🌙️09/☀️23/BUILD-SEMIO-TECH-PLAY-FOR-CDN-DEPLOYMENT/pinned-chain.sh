#!/usr/bin/env bash
# 📌️ Runs the release chain inside the pinned clone with a scrubbed environment. Usage: [SEMIO_PLAY_OFFLOAD=auto|never|distribute] pinned-chain.sh <clone-root>
P="$1"; T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
O="${SEMIO_PLAY_OFFLOAD:-never}"
for v in $(env | /usr/bin/grep -o '^SEMIO_[A-Z_]*='); do unset "${v%=}"; done
unset REPO_ROOT CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR NX_WORKSPACE_ROOT
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$P" SEMIO_BUILD_SERVICE=off BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_REPO_SEAL=live SEMIO_BUILD_MODE=release NATIVE_PLAY_OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/play-cdn-1122/pinned" ATTEMPTS="${ATTEMPTS:-30}"
[ "$O" = auto ] || export SEMIO_OFFLOAD="$O"
cd "$P" && exec bash "$P/$T/native-chain.sh"
