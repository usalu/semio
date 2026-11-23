#!/bin/sh
# 🖍️ Serves the activated Draw react prototype playground on port 6064 through the envelope-issuing launcher.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
mkdir -p "$T/🗑️generated"
DRAW_NATIVE_BUILD_MODE=prototype exec repo tools exec --light bun -- "$T/draw-native-dev.ts" "$T/🗑️generated" "serve-$(date +%H%M%S)" "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts" serve draw react prototype
