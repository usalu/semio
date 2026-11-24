#!/bin/sh
# 🖍️ Runs the English and German Draw browser journeys against the served guest on port 6064.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
RUN="${1:-$(date +%H%M)}"
for locale in ${2:-en de}; do
  DRAW_NATIVE_BUILD_MODE=prototype repo tools exec --light bun -- "$T/draw-native-dev.ts" "$G" "browser-$locale-$RUN" "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts" browser-acceptance --serve http://127.0.0.1:6064/ --locale "$locale" --output "$G/journey-$RUN-$locale" || echo "[DEBUG] journey $locale failed"
done
