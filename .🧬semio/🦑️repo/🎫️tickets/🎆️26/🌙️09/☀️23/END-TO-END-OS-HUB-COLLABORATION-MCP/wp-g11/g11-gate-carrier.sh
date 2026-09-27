#!/bin/zsh
# 🚦️ G11 carrier/budget landing gate (build-landing): plugin + os-mcp `--lib --tests`, hub lib unit tests (`--lib --profile test`).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing"
date; S=$(date +%s)
nice -n 10 cargo check -p semio-framework-plugin -p semio-framework-os-mcp -p semio-framework-plugin-host -p semio-framework-os-kernel --lib --tests --features semio-framework-plugin/artifact-app-testing --message-format short 2>&1
echo "GATE_A_RC=$? secs=$(( $(date +%s)-S ))"
nice -n 10 cargo check -p semio-hub --lib --profile test --message-format short 2>&1
echo "GATE_B_RC=$? secs=$(( $(date +%s)-S ))"; date
