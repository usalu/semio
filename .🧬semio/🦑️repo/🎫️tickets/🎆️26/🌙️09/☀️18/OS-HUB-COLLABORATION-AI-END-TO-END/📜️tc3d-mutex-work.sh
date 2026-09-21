#!/usr/bin/env zsh
# 🔒 The stages 📜️tc3d-hub-boot.sh runs inside ONE fleet wasm mutex hold (preamble rule 27), split
# into its own file so the wrapper takes a single command and every stage's wall time lands in the
# capture. Identical in shape to TC3c's, with TC3d's private target dir and capture names.
#
# Stage 0 is NOT repeated here: 📜️tc3d-hub-boot.sh runs the note wasm32-wasip2 gate as its
# PRE-FLIGHT, outside the mutex, because that check is also the thing that proves a peer's in-flight
# edit is not going to make every plugin build red inside the hold (TC3c lost a whole queue slot to
# exactly that, and TC3d found the stdio `XmlDeclaration { quote }` refactor mid-landing at 17:05).
# Usage (called by 📜️tc3d-hub-boot.sh, never directly): 📜️tc3d-mutex-work.sh <plugin-list> <data-root>
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PLUGINS=${1:-stdio,gis,note}
DATA=${2:-$ROOT/.🧬semio/🌐hub/tc3d-boot}
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
  ( cd "$ROOT" && cargo rustc -p "$crate" --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release ) > "$GEN/tc3d-prebuild-$plugin.txt" 2>&1
  rc=$?
  stamp "stage 1 END $crate exit=$rc"
  tail -12 "$GEN/tc3d-prebuild-$plugin.txt"
  [ $rc -eq 0 ] || exit $rc
done

stamp "stage 2 BEGIN trusted-catalog-bootstrap --packages $PLUGINS"
( cd "$ROOT/🌎️hub/📦️packages/🦀️rust" && OS_HUB_DATA="$DATA" bun ./📜️script.ts trusted-catalog-bootstrap --packages "$PLUGINS" )
rc=$?
stamp "stage 2 END exit=$rc"
exit $rc
