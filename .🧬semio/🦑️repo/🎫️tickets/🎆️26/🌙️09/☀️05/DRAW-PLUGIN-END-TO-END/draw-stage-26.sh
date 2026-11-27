#!/bin/sh
# 🚀️ Stages the fresh Draw component: publishes the shard worker support, prepares and activates the react prototype playground.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
RUN="$1"
DEV="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"
PLUGIN="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"
stage() { name="$1"; shift; echo "[DEBUG] draw stage $name $(date +%T)"; SEMIO_OFFLOAD=never DRAW_NATIVE_BUILD_MODE=prototype repo tools exec --light bun -- "$T/draw-native-dev.ts" "$G" "$name-$RUN" "$@"; }
stage support "$PLUGIN" support
stage prepare "$DEV" prepare draw react prototype
stage activate "$DEV" activate draw react prototype
echo "[DEBUG] draw stage done $(date +%T)"
