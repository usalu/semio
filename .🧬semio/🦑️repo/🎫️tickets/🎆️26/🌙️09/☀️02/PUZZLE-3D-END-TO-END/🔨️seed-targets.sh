#!/bin/zsh
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad
echo "[seed] wasm-release start $(date)"
mkdir -p "$S/target-p3d/wasm32-wasip2"
rsync -a --exclude incremental target/wasm32-wasip2/wasm-release "$S/target-p3d/wasm32-wasip2/" 2>&1 | tail -2
cp target/.rustc_info.json "$S/target-p3d/" 2>/dev/null
echo "[seed] native start $(date)"
rsync -a --exclude incremental --exclude '*.d' target/debug "$S/target-p3d/" 2>&1 | tail -2
echo "[seed] done $(date)"; touch "$S/target-p3d/.seeded"
