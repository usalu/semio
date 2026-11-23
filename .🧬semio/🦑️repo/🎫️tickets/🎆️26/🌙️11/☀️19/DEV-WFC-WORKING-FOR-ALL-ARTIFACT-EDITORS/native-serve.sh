#!/bin/bash
# Serves bitmap, grid2d, wfc2d, grid3d and wfc3d once this ticket's component build appends a new status line.
ROOT="$HOME/Documents/semio"
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS"
S="$T/native-step.sh"
OSDEV="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
STATUS="$T/🗑️generated/component.status"
LOG="$T/🗑️generated/component.log"
WASM="$ROOT/.cache/b/p/m/browser/prototype/modules/🀄️wfc/semio_hub_wfc_component.core.wasm"
MARK="${1:-1529}"
export SEMIO_BUILD_MODE=prototype
export SEMIO_PLUGIN_CATALOG="$ROOT/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
export SEMIO_OFFLOAD=never

for _ in $(seq 1 480); do
  size=$(wc -c < "$STATUS" | tr -d ' ')
  if [ "$size" -gt "$MARK" ]; then
    break
  fi
  sleep 15
done
size=$(wc -c < "$STATUS" | tr -d ' ')
if [ "$size" -le "$MARK" ]; then
  echo "serve-timeout" >> "$T/🗑️generated/serve.status"
  exit 1
fi
tail -1 "$LOG" | grep -q '^rc=0' || { echo "serve-skipped $(tail -1 "$LOG")" >> "$T/🗑️generated/serve.status"; exit 1; }
if [ ! -f "$WASM" ]; then
  echo "serve-missing-wasm" >> "$T/🗑️generated/serve.status"
  exit 1
fi
echo "serve-starting wasm=$(stat -f %m "$WASM") $(stat -f %z "$WASM")" >> "$T/🗑️generated/serve.status"
for variant in bitmap grid2d wfc2d grid3d wfc3d; do
  "$S" "serve-$variant" "$OSDEV" ./📜️script.ts serve "$variant" react prototype &
  echo "started $variant pid $!" >> "$T/🗑️generated/serve.status"
done
wait
