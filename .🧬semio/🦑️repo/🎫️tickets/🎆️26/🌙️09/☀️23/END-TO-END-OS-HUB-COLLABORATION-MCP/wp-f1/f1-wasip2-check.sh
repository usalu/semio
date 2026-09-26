#!/bin/zsh
# 🧱️ F1 — compile-atomic guest-target check of the canvas crate (the shaped-label cache is gated off wasip2; Scene::retained_bytes is not).
cd /Users/ueli/Documents/semio
echo "[f1] wasip2 check start $(date '+%H:%M:%S')"
CARGO_INCREMENTAL=0 nice -n 15 cargo check -p semio-framework-os-infinite --target wasm32-wasip2 --message-format=short 2>&1 | /usr/bin/grep -E "error|canvas/🦀️.rs|Finished|warning: unused" | tail -n 30
echo "[f1] wasip2 check rc ${pipestatus[1]} $(date '+%H:%M:%S')"
