#!/bin/zsh
# [DEBUG] guarded, one at a time: stdio-semio wasm build, its describe, the native laws
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; L=$T/🗑️generated/claude-split
g() { MIN_DISK_GIB=20 $T/claude-resource-guard.sh "$@"; }
g $T/claude-semio-wasm.sh > $L/q4-semio-wasm.log 2>&1; echo "[DEBUG] semio wasm exit $?" >> $L/queue4.out
g $T/claude-describe-wasm.sh > $L/q4-describe.log 2>&1; echo "[DEBUG] describe exit $?" >> $L/queue4.out
g $T/claude-split-tests.sh > $L/q4-tests.log 2>&1; echo "[DEBUG] tests exit $?" >> $L/queue4.out
echo "QUEUE4-DONE" >> $L/queue4.out
