#!/bin/zsh
# 🧪️ C12: one command on the LIVE tree in the native lane (shared build-dir; baselines that tell pre-existing reds from a set's).
# usage: zsh c12-native-run.sh <capture file> <shell command>
OUT="$1"; CMD="$2"
cd /Users/ueli/Documents/semio || exit 1
echo "C12-QUEUED $(date '+%F %T') $CMD" > "$OUT"
zsh .tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "C12-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true; zsh -c "$0"; echo "C12-STEP rc=$?"' "$CMD" >> "$OUT" 2>&1
echo "C12-DONE rc=$? $(date '+%F %T')" >> "$OUT"
