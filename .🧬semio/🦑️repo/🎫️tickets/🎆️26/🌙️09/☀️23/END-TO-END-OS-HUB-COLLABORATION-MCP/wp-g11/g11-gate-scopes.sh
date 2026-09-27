#!/bin/zsh
# 🚦️ G11 per-tool scope gate (native lane, build-fleet-b): os-mcp check, the artifact/ui/policy/conversation laws, quick suite, restage.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
date; S=$(date +%s)
zsh $M native g11 -- zsh -c '
nice -n 10 cargo check -p semio-framework-os-mcp --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "error|could not compile|Finished"; echo "CHECK_RC=${pipestatus[1]}"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && nice -n 10 bun ./📜️script.ts test quick 2>&1 | /usr/bin/grep -E "Summary|FAIL|failed|panicked" | head -30; echo "QUICK_RC=${pipestatus[1]}"
unset CARGO_TARGET_DIR && nice -n 10 bun ./📜️script.ts build 2>&1 | tail -2; echo "RESTAGE_RC=${pipestatus[1]}"
'
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
