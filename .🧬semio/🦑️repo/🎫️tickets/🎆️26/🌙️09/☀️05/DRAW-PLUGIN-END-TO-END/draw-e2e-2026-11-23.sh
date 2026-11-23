#!/bin/sh
# 🖍️ Builds, prepares and activates the current Draw component, then runs the English and German browser journeys against port 6064.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
RUN="${1:-$(date +%H%M)}"
DEV="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"
DRIVER="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts"
MANIFEST="🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml"
BROWSER="✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts"
stage() { name="$1"; shift; echo "[DEBUG] draw e2e $name"; DRAW_NATIVE_BUILD_MODE=prototype repo tools exec --light bun -- "$T/draw-native-dev.ts" "$G" "$name-$RUN" "$@"; }
stage component "$DRIVER" component --manifest "$MANIFEST"
stage prepare "$DEV" prepare draw react prototype
stage activate "$DEV" activate draw react prototype
for locale in en de; do
  stage "browser-$locale" "$BROWSER" browser-acceptance --serve http://127.0.0.1:6064/ --locale "$locale" --output "$G/journey-$RUN-$locale" || echo "[DEBUG] draw e2e journey $locale failed"
done
