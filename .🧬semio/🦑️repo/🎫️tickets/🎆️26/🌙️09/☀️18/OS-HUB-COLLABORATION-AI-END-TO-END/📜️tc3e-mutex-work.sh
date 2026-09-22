#!/usr/bin/env zsh
# 🔒 The stages 📜️tc3e-hub-boot.sh runs inside ONE fleet wasm mutex hold (preamble rule 27).
# Descends from 📜️tc3d-mutex-work.sh with ONE structural change: note is built FIRST and a
# `describe codecs` probe on the freshly built component is a GATE before the other two plugins and
# the hour-long bootstrap. TC3d's hold spent 7 min on stdio, 4 min on note and then 56 min inside
# `trusted-catalog-bootstrap` only to die on that very probe (guest fault
# `interactive-job.close-owned-disposer-missing`, 04:12:44) — the gate turns that into a 5 min loss.
# Usage (called by 📜️tc3e-hub-boot.sh, never directly): 📜️tc3e-mutex-work.sh <plugin-list> <data-root>
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PLUGINS=${1:-note,stdio,gis}
DATA=${2:-$ROOT/.🧬semio/🌐hub/tc3e-boot}
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3d"
stamp() { echo "--- $1 at $(date '+%H:%M:%S') ---"; }

for plugin in ${(s:,:)PLUGINS}; do
  case "$plugin" in
    stdio) crate=semio-s-plugin-stdio ;;
    gis) crate=semio-s-plugin-gis ;;
    note) crate=semio-s-plugin-note ;;
    *) echo "unknown plugin $plugin"; exit 2 ;;
  esac
  stamp "stage 1 BEGIN wasm-release cdylib $crate"
  ( cd "$ROOT" && cargo rustc -p "$crate" --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release ) > "$GEN/tc3e-prebuild-$plugin.txt" 2>&1
  rc=$?
  stamp "stage 1 END $crate exit=$rc"
  tail -12 "$GEN/tc3e-prebuild-$plugin.txt"
  [ $rc -eq 0 ] || exit $rc
  if [ "$plugin" = note ]; then
    stamp "gate BEGIN codec probe on the fresh note component"
    "$CARGO_TARGET_DIR/debug/semio-framework-plugin-describe" codecs \
      "$CARGO_TARGET_DIR/wasm32-wasip2/wasm-release/semio_s_plugin_note.wasm" \
      --kinds s.note.note=note.document --out "$GEN/tc3e-note-codec-rows.json" > "$GEN/tc3e-gate-codecs.txt" 2>&1
    rc=$?
    stamp "gate END exit=$rc"
    tail -20 "$GEN/tc3e-gate-codecs.txt"
    [ $rc -eq 0 ] || exit 77
  fi
done

stamp "stage 2 BEGIN trusted-catalog-bootstrap --packages stdio,gis,note"
( cd "$ROOT/🌎️hub/📦️packages/🦀️rust" && OS_HUB_DATA="$DATA" bun ./📜️script.ts trusted-catalog-bootstrap --packages stdio,gis,note )
rc=$?
stamp "stage 2 END exit=$rc"
exit $rc
