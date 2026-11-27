#!/bin/sh
# 🏁️ Builds, stages and serves the fresh Draw component, then runs the blank-switch matrix, both editor journeys and the Viewer attestation against it.
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
RUN="${1:-fin27}"
S="$G/final-$RUN.status"
say() { echo "[DEBUG] $* $(date +%T)" | tee -a "$S"; }
sh "$T/component-wait-26.sh" "$RUN" || { say build failed; exit 1; }
say built
sh "$T/draw-stage-26.sh" "$RUN" >> "$G/final-$RUN-stage.log" 2>&1 || { say stage failed; exit 1; }
say staged
pkill -f "serve-draw-native.sh" 2>/dev/null
nohup sh "$T/serve-draw-native.sh" > "$G/final-$RUN-serve.log" 2>&1 &
for i in $(seq 1 120); do [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 3 http://127.0.0.1:6064/)" = 200 ] && break; sleep 5; done
say serving
SHA=$(repo tools exec --light bun -- -e'console.log((await Bun.file(".cache/b/p/m/browser/prototype/modules/🖍️draw/🧾️receipt.json").json()).wasmSha256)')
say sha "$SHA"
PLAYWRIGHT_BROWSERS_PATH="$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright" sh "$T/blank-switch-matrix.sh" "$RUN" "artifact,select,inspection,catalogue,artifact2" 0 8 > "$G/final-$RUN-matrix.log" 2>&1
say matrix "$(grep -c 'OK blank' "$G/final-$RUN-matrix.log")/8"
repo tools exec --light bun -- "$T/attest-draw-journeys-26.ts" "j$RUN" "$SHA" 'http://127.0.0.1:6064/' run > "$G/final-$RUN-journeys.log" 2>&1
say journeys exit $?
repo tools exec --light bun -- "$T/attest-draw-journeys-26.ts" "v$RUN" "$SHA" 'http://127.0.0.1:6064/' viewer > "$G/final-$RUN-viewer.log" 2>&1
say viewer exit $?
say done
