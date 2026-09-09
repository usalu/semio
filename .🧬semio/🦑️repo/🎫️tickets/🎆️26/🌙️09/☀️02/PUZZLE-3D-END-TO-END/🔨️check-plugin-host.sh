#!/bin/zsh
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad
until [ -f "$S/target-p3d/.seeded" ]; do sleep 20; done
export CARGO_TARGET_DIR="$S/target-p3d"
export RUSTC_WRAPPER=""
echo "[check] start $(date)"
cargo check -p semio-framework-plugin -j 4 --message-format=short 2>&1 | grep -v "^warning" | tail -30
echo "[check] exit=${pipestatus[1]} $(date)"
