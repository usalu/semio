#!/bin/sh
# 🏗️ HT16-owned copy of 📜️coordinator-hub-run.sh (preamble rule 26 + rule 25/30): the coordinator's
# script is never run in place. Same build + suite, a PRIVATE target dir, HT16's own captures.
cd /Users/ueli/Documents/semio || exit 1
TAG="${1:-}"; SUF=""; [ -n "$TAG" ] && SUF="-$TAG"
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
while pgrep -f "cargo.*nextest.*semio-hub|cargo build -p semio-hub" >/dev/null 2>&1; do sleep 10; done
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-ht16"
echo "BUILD_START $(date '+%H:%M:%S')"
cargo build -p semio-hub --bin os-hub 2>&1 | grep -a -E '^(error|warning: unused)|could not compile|Finished|^  -->' | grep -a -v '^warning' | head -80
ls -la "$CARGO_TARGET_DIR/debug/os-hub" && echo "BIN_DONE $(date '+%H:%M:%S')"
echo "NEXTEST_START $(date '+%H:%M:%S')"
cargo nextest run -p semio-hub --no-fail-fast --profile long --status-level all --failure-output final > "$TICKET/🗑️generated/ht16-hub-nextest-full${SUF}.txt" 2>&1
grep -a -E '^\s+(PASS|FAIL|TIMEOUT|SIGABRT|SIGSEGV|LEAK)|Summary|^error|could not compile' "$TICKET/🗑️generated/ht16-hub-nextest-full${SUF}.txt" > "$TICKET/🗑️generated/ht16-hub-nextest-latest${SUF}.txt"
echo "NEXTEST_DONE $(date '+%H:%M:%S')"
grep -a Summary "$TICKET/🗑️generated/ht16-hub-nextest-latest${SUF}.txt"
