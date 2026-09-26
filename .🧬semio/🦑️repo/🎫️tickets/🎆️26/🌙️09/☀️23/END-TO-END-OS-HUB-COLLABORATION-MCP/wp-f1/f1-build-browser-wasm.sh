#!/bin/zsh
# 🧱️ F1 — builds ONE browser wasm bundle through nx (framework-editor-rs | framework-surface-rs); run under the wasmshort lane.
cd /Users/ueli/Documents/semio
target="$1"
echo "[f1] build $target start $(date '+%H:%M:%S')"
CARGO_INCREMENTAL=0 NX_DAEMON=false nice -n 15 bun nx run "$target" --excludeTaskDependencies --outputStyle=stream --skip-nx-cache
echo "[f1] build $target rc $? $(date '+%H:%M:%S')"
