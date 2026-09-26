#!/bin/zsh
# 🧱️ F1 — compile-atomic wasm32 checks of the shaped-label cache (canvas text): the browser target the editor/surface
# bundles build for, and the plugin guest target (every new item is gated off wasip2; the check proves it).
cd /Users/ueli/Documents/semio
echo "[f1] start $(date '+%H:%M:%S')"
CARGO_INCREMENTAL=0 nice -n 15 cargo check -p semio-framework-editor --target wasm32-unknown-unknown --message-format=short 2>&1 | tail -n 40
echo "[f1] editor wasm32-unknown-unknown rc ${pipestatus[1]} $(date '+%H:%M:%S')"
CARGO_INCREMENTAL=0 nice -n 15 cargo check -p semio-framework-os-infinite --target wasm32-wasip2 --message-format=short 2>&1 | tail -n 40
echo "[f1] infinite wasm32-wasip2 rc ${pipestatus[1]} $(date '+%H:%M:%S')"
