#!/bin/zsh
# ⚖️ LW1: run one owner's law command in the native lane (build-fleet-b, private target under the hub dir, CARGO_INCREMENTAL=0,
# NX_DAEMON=false) and append START/END rc lines to its capture under `.🧬semio/🌐hub/s14-lw1-logs/`.
# usage: zsh lw1-law.sh <capture-name> <cwd> -- <command …>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket
name="$1"; dir="$2"; shift 2; [ "$1" = "--" ] && shift
out="$R/.🧬semio/🌐hub/s14-lw1-logs/$name.txt"
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_BACKTRACE=0
export CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$R/.🧬semio/🌐hub/s14-lw1-target"
echo "LW1-QUEUED $(date '+%F %T') cwd=$dir cmd=$*" >> "$out"
cd "$dir" || { echo "LW1-END rc=2 (cwd) $(date '+%F %T')" >> "$out"; exit 2; }
zsh "$T/📜️fleet-mutex.sh" native lw1 -- zsh -c 'echo "LW1-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; s=$(date +%s); nice -n 15 "$@"; rc=$?; echo "LW1-RC rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' lw1 "$@" >> "$out" 2>&1
rc=$?
echo "LW1-END rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
