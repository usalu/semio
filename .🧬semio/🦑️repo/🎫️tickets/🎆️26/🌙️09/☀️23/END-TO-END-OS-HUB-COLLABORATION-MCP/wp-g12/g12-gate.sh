#!/bin/zsh
# 🚦️ G12 os-mcp gate (native lane, build-fleet-b, private target): check --lib --tests → Rust quick suite → gateway binary
# copied (never overwritten in place) to .🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-<tag>.
# usage: zsh g12-gate.sh <tag>
TAG="${1:?tag}"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g12/target
export G12_TAG="$TAG"
date; S=$(date +%s)
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native g12 -- zsh -c '
echo "LANE_START $(date +%H:%M:%S)"
nice -n 15 cargo check -p semio-framework-os-mcp --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|could not compile|Finished|warning: unused" | head -60; echo "CHECK_RC=${pipestatus[1]} $(date +%H:%M:%S)"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && nice -n 15 bun ./📜️script.ts test quick 2>&1 | /usr/bin/grep -E "Summary|FAIL|failed|panicked|error\[" | head -40; echo "QUICK_RC=${pipestatus[1]} $(date +%H:%M:%S)"
cd /Users/ueli/Documents/semio && nice -n 15 cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short 2>&1 | tail -2; echo "BUILD_RC=${pipestatus[1]} $(date +%H:%M:%S)"
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-$G12_TAG"
rm -f "$OUT" && cp "$CARGO_TARGET_DIR/debug/semio-os-mcp" "$OUT" && codesign -f -s - "$OUT" 2>/dev/null; ls -la "$OUT"
'
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
