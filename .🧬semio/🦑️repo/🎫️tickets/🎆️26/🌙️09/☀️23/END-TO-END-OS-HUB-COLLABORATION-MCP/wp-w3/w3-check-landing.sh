#!/bin/zsh
# 🧪 W3 compile-atomic native check of the item 3/4 landings on the landing build-dir (rule 27): kernel (new derive input), MCP, os run, plugin host.
cd /Users/ueli/Documents/semio
until [ $(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ') -le 14 ]; do sleep 30; done
echo "[w3] START $(date '+%H:%M:%S')"
CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing" cargo check -p semio-framework-os-kernel -p semio-framework-os-mcp -p semio-framework-os-run -p semio-framework-plugin-host --lib --tests
echo "[w3] END rc=$? $(date '+%H:%M:%S')"
