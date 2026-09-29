#!/bin/zsh
# 🧪️ C12: live-tree baseline (native lane) of the SDK laws the T6 overlay proof ran, to tell pre-existing reds from the set's.
# usage: zsh sdk-baseline.sh <capture file> <nextest filter expression>
OUT="$1"; FILTER="$2"
cd /Users/ueli/Documents/semio || exit 1
echo "C12-QUEUED $(date '+%F %T') filter=$FILTER" > "$OUT"
zsh .tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "C12-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true; cargo nextest run --profile long --no-fail-fast -p semio-framework-plugin --lib -E "$0"; echo "C12-STEP sdk rc=$?"' "$FILTER" >> "$OUT" 2>&1
echo "C12-DONE rc=$? $(date '+%F %T')" >> "$OUT"
