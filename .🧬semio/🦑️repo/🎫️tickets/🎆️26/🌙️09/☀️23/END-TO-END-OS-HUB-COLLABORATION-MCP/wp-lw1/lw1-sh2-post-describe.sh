#!/bin/zsh
# ⚖️ LW1 → SH2 after the space describe regen: plugin-space lib (descriptor_is_fresh) + interactive-job-catalog source oracle.
R=/Users/ueli/Documents/semio
cargo test --offline --no-fail-fast -p semio-s-plugin-space --lib; echo "LW1-STEP plugin-space-lib rc=$?"
cd "$R/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust" && bun ./📜️script.ts interactive-job-catalog-check; echo "LW1-STEP interactive-job-catalog-check rc=$?"
