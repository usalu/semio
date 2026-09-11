#!/bin/sh
# Probe: two concurrent cargo invocations sharing one build-dir under -Zfine-grain-locking.
set -u
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/NX-COMPLETE-TASK-CACHING/🗑️generated"
B="$PWD/$T/probe-build"
O="$PWD/$T/probe-target"
export RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_INCREMENTAL=0
export CARGO_BUILD_BUILD_DIR="$B" CARGO_TARGET_DIR="$O"
F="-Zfine-grain-locking -Zbuild-dir-new-layout -Zchecksum-freshness"
start=$(date +%s)
cargo check $F -p "${1:-semio-framework-value-derive}" --message-format short > "$T/probe-a.txt" 2>&1 &
A=$!
sleep 2
cargo check $F -p "${2:-semio-framework-hash}" --message-format short > "$T/probe-b.txt" 2>&1 &
Bp=$!
wait $A; ea=$?
wait $Bp; eb=$?
echo "a=$ea b=$eb elapsed=$(( $(date +%s) - start ))s"
grep -h "Blocking\|waiting\|error" "$T/probe-a.txt" "$T/probe-b.txt" | head
tail -2 "$T/probe-a.txt" "$T/probe-b.txt"
ls "$B" | head
