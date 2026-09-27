#!/bin/zsh
# 🚦️ G11 directory-grant landing gate (native lane, build-fleet-b): kernel + os-mcp `--lib --tests` check, the kernel grant law,
# the MCP dial-refusal law, then the os-mcp restage.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
date; S=$(date +%s)
zsh $M native g11 -- zsh -c '
nice -n 10 cargo check -p semio-framework-os-kernel -p semio-framework-os-mcp --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^(error|warning: unused)|error\[|could not compile|Finished"; echo "CHECK_RC=${pipestatus[1]}"
nice -n 10 cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- the_space_wide_directory_grant open_scoped_stream stream_turn 2>&1 | /usr/bin/grep -E "^test |test result|panicked"; echo "LAW_KERNEL_RC=${pipestatus[1]}"
nice -n 10 cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- a_directory_dial_refusal workspace::remote:: 2>&1 | /usr/bin/grep -E "^test |test result|panicked" | tail -30; echo "LAW_MCP_RC=${pipestatus[1]}"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && unset CARGO_TARGET_DIR && nice -n 10 bun ./📜️script.ts build 2>&1 | tail -3; echo "RESTAGE_RC=${pipestatus[1]}"
'
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
