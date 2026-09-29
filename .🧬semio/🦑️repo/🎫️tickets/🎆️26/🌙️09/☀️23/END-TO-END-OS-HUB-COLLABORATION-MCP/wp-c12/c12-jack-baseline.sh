#!/bin/zsh
# 🧪️ C12: live-tree baseline of trinity jack's feature-gated editor laws (incl. the ledger twin) in the native lane — the pre-T6 status
# the overlay proof compares against. usage: zsh c12-jack-baseline.sh <capture file>
OUT="$1"
cd /Users/ueli/Documents/semio || exit 1
echo "C12-QUEUED $(date '+%F %T')" > "$OUT"
zsh .tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "C12-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true; cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-trinity-jack --lib --features component-app-assembly; echo "C12-STEP jack rc=$?"' >> "$OUT" 2>&1
echo "C12-DONE rc=$? $(date '+%F %T')" >> "$OUT"
