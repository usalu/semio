#!/usr/bin/env zsh
# 🔒 The three stages 📜️tc3c-hub-boot.sh runs inside ONE fleet wasm mutex hold. Split into its own
# file so the mutex wrapper takes a single command and every stage's wall time lands in the capture.
# Usage (called by 📜️tc3c-hub-boot.sh, never directly): 📜️tc3c-mutex-work.sh <plugin-list> <data-root>
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PLUGINS=${1:-stdio,gis,note}
DATA=${2:-$ROOT/.🧬semio/🌐hub/tc3c-boot}
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3c"
stamp() { echo "--- $1 at $(date '+%H:%M:%S') ---"; }

stamp "stage 0 BEGIN note wasm32-wasip2 codec-guest compile gate (TC3b §6a)"
# 🧩️ No `--features component-app-assembly`: `semio-s-plugin-note`'s Cargo.toml declares NO
# `[features]` table at all, so naming one aborts with "does not contain this feature" (measured
# 13:39:13, exit 101, and it cost a whole mutex hold). The guest halves of `world actor`'s `codec`
# interface are `cfg(all(target_arch = "wasm32", target_env = "p2"))`, which the plain wasm32-wasip2
# check compiles — the feature was never what gated them.
( cd "$ROOT" && cargo check -p semio-s-plugin-note --target wasm32-wasip2 ) > "$GEN/tc3c-note-wasm-check.txt" 2>&1
gate=$?
stamp "stage 0 END exit=$gate"
tail -25 "$GEN/tc3c-note-wasm-check.txt"
[ $gate -eq 0 ] || { echo "note wasm codec gate FAILED — refusing to build components"; exit $gate; }

for plugin in ${(s:,:)PLUGINS}; do
  case "$plugin" in
    stdio) crate=semio-s-plugin-stdio ;;
    gis) crate=semio-s-plugin-gis ;;
    note) crate=semio-s-plugin-note ;;
    *) echo "unknown plugin $plugin"; exit 2 ;;
  esac
  stamp "stage 1 BEGIN cold wasm-release cdylib $crate"
  ( cd "$ROOT" && cargo rustc -p "$crate" --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release ) > "$GEN/tc3c-prebuild-$plugin.txt" 2>&1
  rc=$?
  stamp "stage 1 END $crate exit=$rc"
  tail -12 "$GEN/tc3c-prebuild-$plugin.txt"
  [ $rc -eq 0 ] || exit $rc
done

if [ "${TC3C_WAIT_FOR_SOURCE:-0}" = "1" ]; then
  waited=0
  while [ ! -f "$GEN/tc3c-source-ready.txt" ] && [ "$waited" -lt 2700 ]; do
    sleep 30
    waited=$((waited + 30))
  done
  stamp "stage 2 source-ready wait ${waited}s present=$([ -f "$GEN/tc3c-source-ready.txt" ] && echo yes || echo no)"
  [ -f "$GEN/tc3c-source-ready.txt" ] || { echo "builder rewrite never landed — releasing the mutex without bootstrapping"; exit 3; }
fi

stamp "stage 2 BEGIN trusted-catalog-bootstrap --packages $PLUGINS"
( cd "$ROOT/🌎️hub/📦️packages/🦀️rust" && OS_HUB_DATA="$DATA" bun ./📜️script.ts trusted-catalog-bootstrap --packages "$PLUGINS" )
rc=$?
stamp "stage 2 END exit=$rc"
exit $rc
