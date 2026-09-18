#!/bin/zsh
# 🀄️ Restages the react dev runtime for every wfc playground variant (one shared wasm component, five receipts). Retries on SIGKILL (swap).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
LOGDIR="${1:-/tmp}"
for variant in ${=2:-bitmap grid2d wfc2d grid3d wfc3d}; do
  LOG="$LOGDIR/activate-$variant.log"
  ok=0
  for attempt in 1 2 3 4; do
    bun nx run "@semio-tech/framework-os-dev:activate-$variant-react-dev" --output-style=static > "$LOG" 2>&1
    code=$?
    if [ $code -eq 0 ]; then echo "ACTIVATE OK $variant attempt $attempt" >> "$LOG"; ok=1; break; fi
    if grep -qiE 'SIGKILL|signal: 9|killed' "$LOG"; then echo "ACTIVATE SIGKILL $variant attempt $attempt, retrying" >> "$LOG"; sleep 90; continue; fi
    echo "ACTIVATE FAILED $variant code $code attempt $attempt" >> "$LOG"; break
  done
  [ $ok -eq 1 ] || exit 1
done
echo "ALL ACTIVATED" >> "$LOGDIR/activate-all.log"
