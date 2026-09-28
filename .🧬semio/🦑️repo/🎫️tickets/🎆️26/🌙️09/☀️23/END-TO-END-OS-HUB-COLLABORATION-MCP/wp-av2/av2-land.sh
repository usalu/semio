#!/bin/zsh
# 🛬️ AV2 window-3 landing of the `media.video-render` set, compile-atomic: dry run → write → native lane check (every touched
# crate, --lib --tests) → on red: revert at once (av2-apply.py --revert) and stop; on green: laws (native tests, TS laws, tsc).
# wasm32 of the animate guest + the serve boot to Home run after this script (wasm lane + one dev serve).
#   zsh .tmp-ticket/wp-av2/av2-land.sh <capture-dir>
setopt no_bg_nice
ROOT=/Users/ueli/Documents/semio
W="$ROOT/.tmp-ticket/wp-av2"
OUT="${1:-$W/generated/land}"
MUTEX="$ROOT/.tmp-ticket/📜️fleet-mutex.sh"
mkdir -p "$OUT"
cd "$ROOT" || exit 2
export CARGO_BUILD_BUILD_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$W/target" CARGO_INCREMENTAL=0 NX_DAEMON=false
CRATES=(-p semio-framework -p semio-framework-raster -p semio-framework-plugin -p semio-framework-plugin-host -p semio-s-artifact-animate-presentation -p semio-s-plugin-animate)
log() { echo "[av2-land] $* $(date '+%F %T')"; }
python3 "$W/av2-apply.py" > "$OUT/dry-run.txt" 2>&1 || { log "dry run refused"; cat "$OUT/dry-run.txt"; exit 1; }
python3 "$W/av2-apply.py" --write > "$OUT/write.txt" 2>&1 || { log "write refused"; cat "$OUT/write.txt"; exit 1; }
log "written"
zsh "$MUTEX" native av2 -- nice -n 15 cargo check $CRATES --lib --tests > "$OUT/check.txt" 2>&1
rc=$?
log "native check rc=$rc warnings=$(/usr/bin/grep -c '^warning' "$OUT/check.txt") errors=$(/usr/bin/grep -c '^error' "$OUT/check.txt")"
if [ $rc -ne 0 ]; then
  python3 "$W/av2-apply.py" --revert > "$OUT/revert.txt" 2>&1
  log "REVERTED rc=$? ($(tail -1 "$OUT/revert.txt"))"
  exit 1
fi
zsh "$MUTEX" native av2 -- zsh -c '
  nice -n 15 cargo test -p semio-framework --lib --no-fail-fast -- video_render > "$0/test-kernel.txt" 2>&1; echo "kernel=$?"
  nice -n 15 cargo test -p semio-framework-raster --lib --no-fail-fast -- video > "$0/test-raster.txt" 2>&1; echo "raster=$?"
  nice -n 15 cargo test -p semio-framework-plugin --lib --no-fail-fast -- wire_effect_round_trip > "$0/test-plugin.txt" 2>&1; echo "plugin=$?"
  nice -n 15 cargo test -p semio-s-artifact-animate-presentation --lib --no-fail-fast -- export_video program_unit a_deck a_tileless a_stated every_command retained > "$0/test-animate.txt" 2>&1; echo "animate=$?"
' "$OUT" > "$OUT/tests.txt" 2>&1
log "native tests $(tr '\n' ' ' < "$OUT/tests.txt")"
bun "$W/av2-laws.ts" "$ROOT" > "$OUT/laws.txt" 2>&1
log "TS laws rc=$? $(tail -1 "$OUT/laws.txt")"
(cd "$W/tsc" && nice -n 12 "$ROOT/node_modules/.bin/tsc" -p tsconfig-live.json --pretty false > "$OUT/tsc.txt" 2>&1; echo "exit=$?" >> "$OUT/tsc.txt")
log "tsc $(tail -1 "$OUT/tsc.txt") errors=$(/usr/bin/grep -c 'error TS' "$OUT/tsc.txt")"
log "DONE"
