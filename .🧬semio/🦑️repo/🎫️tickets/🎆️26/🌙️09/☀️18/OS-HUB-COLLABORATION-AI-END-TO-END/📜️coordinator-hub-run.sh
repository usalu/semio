#!/bin/sh
# 🏗️ Coordinator-owned hub build + suite (preamble rule 26): one builder for semio-hub.
cd /Users/ueli/Documents/semio || exit 1
TAG="${1:-}"; SUF=""; [ -n "$TAG" ] && SUF="-$TAG"
while pgrep -f "cargo.*nextest.*semio-hub|cargo build -p semio-hub" >/dev/null 2>&1; do sleep 10; done
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub"
echo "BUILD_START $(date '+%H:%M:%S')"
cargo build -p semio-hub --bin os-hub 2>&1 | grep -a -E '^(error|warning: unused)|could not compile|Finished|^  -->' | grep -a -v '^warning' | head -80
ls -la "$CARGO_TARGET_DIR/debug/os-hub" && echo "BIN_DONE $(date '+%H:%M:%S')"
cargo nextest run -p semio-hub --no-fail-fast --profile long --status-level all --failure-output final > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/coordinator-hub-nextest-full${SUF}.txt" 2>&1
grep -a -E '^\s+(PASS|FAIL|TIMEOUT|SIGABRT|SIGSEGV)|Summary|^error|could not compile' "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/coordinator-hub-nextest-full${SUF}.txt" > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/coordinator-hub-nextest-latest${SUF}.txt"
echo "NEXTEST_DONE $(date '+%H:%M:%S')"
grep -a Summary "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/coordinator-hub-nextest-latest${SUF}.txt"
