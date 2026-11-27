#!/bin/sh
# 🔁️ Retries the Draw component driver while the host warden cancels units under memory pressure; any other outcome ends the loop.
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
DRIVER="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts"
MANIFEST="🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml"
for attempt in $(seq 1 8); do
  log="$G/draw-native-component-r$attempt-$1.log"; : > "$log"
  SEMIO_OFFLOAD=never DRAW_NATIVE_BUILD_MODE=prototype repo tools exec --light bun -- "$T/draw-native-dev.ts" "$G" "component-r$attempt-$1" "$DRIVER" component --manifest "$MANIFEST" > "$G/retry-r$attempt-$1.out" 2>&1
  echo "[DEBUG] attempt $attempt exit $? $(date +%T)" >> "$G/retry-$1.attempts"
  grep -aq "built, described and materialized" "$log" && exit 0
  grep -aq "ended by SIGTERM\|cancelled by the host warden\|cannot update the lock file\|Descriptor emitter is stale\|admission closed\|Native build failed\|Command failed: cargo" "$log" || exit 1
done
exit 2
