#!/bin/zsh
# ⚖️ LW1 → SH2 laws (T3 sh2-space-home + sh2-b1, runbook step 3): os-config lib, space-home lib (± component-app-assembly), plugin-space lib.
cargo test --offline --no-fail-fast -p semio-framework-os-config --lib; echo "LW1-STEP os-config-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-s-artifact-space-home --features component-app-assembly --lib; echo "LW1-STEP space-home-caa-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-s-artifact-space-home --lib; echo "LW1-STEP space-home-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-s-plugin-space --lib; echo "LW1-STEP plugin-space-lib rc=$?"
