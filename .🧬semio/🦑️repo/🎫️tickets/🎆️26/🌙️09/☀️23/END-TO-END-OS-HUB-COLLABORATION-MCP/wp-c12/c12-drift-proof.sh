#!/bin/zsh
# 🧪️ C12 14c rule-22 proof of the test-only drift fix (`c12-writer-test-drift-edit.py`): `--lib` laws of the named crates in the native
# lane, the nextest recipe LW1 ran (`s14-lw1-logs/c12-nextest-1.txt`). usage: zsh c12-drift-proof.sh <capture file> <crate…>
OUT="$1"; shift
cd /Users/ueli/Documents/semio || exit 1
echo "C12-QUEUED $(date '+%F %T') crates=$*" > "$OUT"
zsh .tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "C12-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true; for crate in "$@"; do cargo nextest run --profile long --no-fail-fast -p "$crate" --lib; echo "C12-STEP $crate rc=$?"; done' c12 "$@" >> "$OUT" 2>&1
echo "C12-DONE rc=$? $(date '+%F %T')" >> "$OUT"
