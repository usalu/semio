#!/bin/zsh
# 🧪️ E1 re-proof on the post-T6 tree: clone `s14-u6-e1o` (live clone + `u6-e1-paged-docx.py --write`), PRIVATE build/target dirs,
# run through the overlay lane: docx lib WITH `component-app-assembly` (the 5 MB set-page law), stdio contract + semio libs, kernel
# `store` laws (the two `retained_clone` laws are red on live without E1 — `e1-live-kernel-retained-clone.txt`).
R=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-e1o
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-e1obuild CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-e1otarget
cd "$R" || exit 2
echo "DOCX $(date +%T)"
cargo test --offline --lib --no-fail-fast -p semio-s-artifact-stdio-docx --features component-app-assembly 2>&1 | tee $L/e1t6-docx.txt | /usr/bin/grep -E "^test .* FAILED$|^test result|^error" | head -40
echo "STDIO $(date +%T)"
cargo test --offline --lib --no-fail-fast -p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-semio 2>&1 | tee $L/e1t6-stdio.txt | /usr/bin/grep -E "^test .* FAILED$|^test result|^error" | head -40
echo "KERNEL $(date +%T)"
cargo test --offline --lib --no-fail-fast -p semio-framework-os-kernel store 2>&1 | tee $L/e1t6-kernel.txt | /usr/bin/grep -E "^test .* FAILED$|^test result|^error" | head -40
echo "E1T6-DONE $(date +%T)"
