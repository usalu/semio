#!/bin/zsh
# 🔁 Restage the generation3d React guest after the procedural rename fix-forward (FlowHostSnapshot /
# host_snapshot / a single artifact_schema). nx cache skipped: the plugin's nx hash does not see every
# Rust leaf, so a cached `component-dev` serves a guest older than the tree.
DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/rename-fixforward"
LOG="$DIR/restage.txt"
mkdir -p "$DIR"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
for attempt in 1 2 3; do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  grep -aq "error\[E" "$LOG" && break
done
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/" >> "$LOG" 2>&1
echo "RESTAGE-FIXFORWARD-DONE rc=$rc" >> "$LOG"
