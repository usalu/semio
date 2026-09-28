#!/bin/zsh
# 🚦️ G12 os-mcp gate (native lane, build-fleet-b, private target under .🧬semio/🌐hub — rule 26): check --lib --tests (skipped with
# G12_SKIP_CHECK=1) → the named lib-law filters → gateway build only with G12_BUILD=1. Binary copied (never overwritten in place, only after a green build) to .🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-<tag>.
# usage: zsh g12-gate-mcp.sh <tag> <law filter>...
TAG="${1:?tag}"; shift
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-g12-target"
export G12_TAG="$TAG" G12_FILTERS="$*" G12_BUILD="${G12_BUILD:-}" G12_SKIP_CHECK="${G12_SKIP_CHECK:-}"
date; S=$(date +%s)
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native g12 -- zsh -c '
echo "LANE_START $(date +%H:%M:%S)"
[ -n "$G12_SKIP_CHECK" ] || { nice -n 15 cargo check -p semio-framework-os-mcp --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|could not compile|Finished|generated [0-9]+ warning" | head -40; echo "MCP_CHECK_RC=${pipestatus[1]} $(date +%H:%M:%S)"; }
for FILTER in ${=G12_FILTERS}; do
  nice -n 15 cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- "$FILTER" 2>&1 | /usr/bin/grep -E "^test |test result|panicked|error\[" | head -60; echo "LAWS[$FILTER]_RC=${pipestatus[1]} $(date +%H:%M:%S)"
done
[ -n "$G12_BUILD" ] || exit 0
nice -n 15 cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short 2>&1 | tail -2; BUILD=${pipestatus[1]}; echo "BUILD_RC=$BUILD $(date +%H:%M:%S)"
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-$G12_TAG"
if [ "$BUILD" = 0 ]; then rm -f "$OUT" && cp "$CARGO_TARGET_DIR/debug/semio-os-mcp" "$OUT" && codesign -f -s - "$OUT" 2>/dev/null; ls -la "$OUT"; fi
'
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
