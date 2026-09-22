#!/bin/sh
# 🏗️ DB4-owned copy of 📜️ht16-hub-run.sh (preamble rule 26 + 25/30): neither the coordinator's nor
# HT16's script is ever run in place. Same build + suite, a PRIVATE target dir, DB4's own captures,
# and the two live database drivers compiled in so the postgres/neo4j lanes actually execute.
cd /Users/ueli/Documents/semio || exit 1
TAG="${1:-}"; SUF=""; [ -n "$TAG" ] && SUF="-$TAG"
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-db4"
FEATURES="postgres,neo4j"
echo "BUILD_START $(date '+%H:%M:%S')"
cargo build -p semio-hub --bin os-hub --features "$FEATURES" 2>&1 | grep -a -E '^(error|warning: unused)|could not compile|Finished|^  -->' | grep -a -v '^warning' | head -80
ls -la "$CARGO_TARGET_DIR/debug/os-hub" && echo "BIN_DONE $(date '+%H:%M:%S')"
echo "NEXTEST_START $(date '+%H:%M:%S')"
cargo nextest run -p semio-hub --features "$FEATURES" --no-fail-fast --profile long --status-level all --failure-output final > "$TICKET/🗑️generated/db4-hub-nextest-full${SUF}.txt" 2>&1
grep -a -E '^\s+(PASS|FAIL|TIMEOUT|SIGABRT|SIGSEGV|LEAK)|Summary|^error|could not compile' "$TICKET/🗑️generated/db4-hub-nextest-full${SUF}.txt" > "$TICKET/🗑️generated/db4-hub-nextest-latest${SUF}.txt"
echo "NEXTEST_DONE $(date '+%H:%M:%S')"
grep -a Summary "$TICKET/🗑️generated/db4-hub-nextest-latest${SUF}.txt"
