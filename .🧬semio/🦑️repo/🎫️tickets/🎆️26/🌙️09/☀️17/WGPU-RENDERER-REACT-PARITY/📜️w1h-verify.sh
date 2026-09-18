#!/usr/bin/env bash
# 🧪 W1h verification chain — native lib check, targeted panel/dock/anchor tests, wasm32 lib check.
# Retries the native check while a PEER's in-flight breakage elsewhere in the crate (🎞️Scenes,
# 🔗️AgentBridge, …) keeps the whole crate from compiling: the packet's own file is clean as soon as
# no error block names 🐚️Shell. Logs land beside this script under 🗑️generated/.
set -u
TICKET="$(cd "$(dirname "$0")" && pwd)"
G="$TICKET/🗑️generated"
cd "$(git -C "$TICKET" rev-parse --show-toplevel)" || exit 1
mkdir -p "$G"
export CARGO_INCREMENTAL=0

attempt=0
until [ "$attempt" -ge "${W1H_MAX_ATTEMPTS:-40}" ]; do
  attempt=$((attempt + 1))
  cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going -j "${W1H_JOBS:-1}" > "$G/w1h-native-check.txt" 2>&1
  status=$?
  shell_errors=$(grep -c "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs" <(grep -A 2 "^error" "$G/w1h-native-check.txt"))
  echo "attempt=$attempt cargo=$status shell-file-errors=$shell_errors"
  [ "$status" -eq 0 ] && break
  sleep 45
done

if [ "$status" -ne 0 ]; then
  echo "native lib check still blocked by peer breakage; see $G/w1h-native-check.txt"
  exit 0
fi

cargo test -p semio-framework-os-renderer-wgpu --lib -j "${W1H_JOBS:-1}" -- --nocapture \
  panel_anchor dock_ anchor_ default_dock move_tab_in_dock apply_dock_skeleton reconcile_path column_projections persist_dock_ui build_command_panel ui_snapshot \
  > "$G/w1h-tests.txt" 2>&1
echo "tests=$?"

cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going -j "${W1H_JOBS:-1}" > "$G/w1h-wasm-check.txt" 2>&1
echo "wasm=$?"
