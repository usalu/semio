#!/bin/zsh
# 🔁️ W4: reproduce the two component-dev failures of the session-13 final chain alone (robotic, flow text), one after the other,
# inside ONE fleet wasm hold, default build-dir (the chain's), so the real rustc error is not lost in nx's interleaved output.
# usage: zsh w4-repro.sh <capture dir>
setopt no_bg_nice
OUT="$1"
cd /Users/ueli/Documents/semio || exit 1
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
for project in process-extension-robotic-rust flow-extension-text-rust; do
  echo "[w4-repro] START $project $(date '+%F %T')"
  s=$(date +%s)
  bun nx run "@semio-tech/${project}:component-dev" --skip-nx-cache --outputStyle=stream > "$OUT/repro-$project.txt" 2>&1
  echo "[w4-repro] END $project rc=$? wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
done
