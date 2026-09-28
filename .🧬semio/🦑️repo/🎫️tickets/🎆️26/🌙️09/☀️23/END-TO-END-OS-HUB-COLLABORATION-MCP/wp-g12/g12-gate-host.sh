#!/bin/zsh
# 🚦️ G12 plugin-host + os-mcp gate (native lane, build-fleet-b, private target): plugin-host check --lib --tests → the
# shared-wasmtime-engine laws → os-mcp check --lib --tests → os-mcp shell_channel laws → gateway binary copied (never overwritten in place) to
# .🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-<tag>.  usage: zsh g12-gate-host.sh <tag>
TAG="${1:?tag}"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g12/target
export G12_TAG="$TAG"
date; S=$(date +%s)
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native g12 -- zsh -c '
echo "LANE_START $(date +%H:%M:%S)"
nice -n 15 cargo check -p semio-framework-plugin-host --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^(error|warning)|error\[|: (error|warning)|could not compile|Finished" | head -40; echo "HOST_CHECK_RC=${pipestatus[1]} $(date +%H:%M:%S)"
nice -n 15 cargo test -p semio-framework-plugin-host --lib --no-fail-fast -- shared_wasmtime_engine 2>&1 | /usr/bin/grep -E "^test |test result|panicked|error\[" | head -40; echo "HOST_LAWS_RC=${pipestatus[1]} $(date +%H:%M:%S)"
nice -n 15 cargo check -p semio-framework-os-mcp --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|could not compile|Finished" | head -40; echo "MCP_CHECK_RC=${pipestatus[1]} $(date +%H:%M:%S)"
nice -n 15 cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- shell_channel 2>&1 | /usr/bin/grep -E "^test |test result|panicked|error\[" | head -40; echo "MCP_CHANNEL_LAWS_RC=${pipestatus[1]} $(date +%H:%M:%S)"
nice -n 15 cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short 2>&1 | tail -2; echo "BUILD_RC=${pipestatus[1]} $(date +%H:%M:%S)"
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-$G12_TAG"
rm -f "$OUT" && cp "$CARGO_TARGET_DIR/debug/semio-os-mcp" "$OUT" && codesign -f -s - "$OUT" 2>/dev/null; ls -la "$OUT"
'
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
