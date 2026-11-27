#!/bin/zsh
# [DEBUG] after the priority stdio-office rebuild: build and describe stdio-semio, then run the native laws, one guarded job at a time
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; L=$T/🗑️generated/claude-split

g() { MIN_DISK_GIB=20 $T/claude-resource-guard.sh "$@"; }
g $T/claude-describe-wasm.sh > $L/q-describe1.log 2>&1; echo "[DEBUG] describe1 exit $?" >> $L/queue3.out
g $T/claude-semio-wasm.sh > $L/q-semio-wasm.log 2>&1; echo "[DEBUG] semio wasm exit $?" >> $L/queue3.out
g $T/claude-describe-wasm.sh > $L/q-describe2.log 2>&1; echo "[DEBUG] describe2 exit $?" >> $L/queue3.out
g $T/claude-split-tests.sh > $L/q-tests.log 2>&1; echo "[DEBUG] tests exit $?" >> $L/queue3.out
echo "QUEUE3-DONE" >> $L/queue3.out
