#!/bin/sh
# 🚀️ Publishes the shard worker, prepares and activates the already staged Draw component, then runs both journeys.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
RUN="$1"
DEV="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"
PLUGIN="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"
stage() { name="$1"; shift; echo "[DEBUG] draw e2e $name"; DRAW_NATIVE_BUILD_MODE=prototype repo tools exec --light bun -- "$T/draw-native-dev.ts" "$G" "$name-$RUN" "$@"; }
stage support "$PLUGIN" support
stage prepare "$DEV" prepare draw react prototype
stage activate "$DEV" activate draw react prototype
sh "$T/draw-journeys.sh" "$RUN"
