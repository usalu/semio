#!/bin/zsh
# ⚖️ LW1: one wasm-lane job (default build-dir, as the chain; describe/materialize regen) with LW1-START/RC/END lines in its capture.
# usage: zsh lw1-wasm.sh <capture-name> <cwd> -- <command …>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket
name="$1"; dir="$2"; shift 2; [ "$1" = "--" ] && shift
out="$R/.🧬semio/🌐hub/s14-lw1-logs/$name.txt"
export CARGO_INCREMENTAL=0 NX_DAEMON=false
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
echo "LW1-QUEUED $(date '+%F %T') wasm cwd=$dir cmd=$*" >> "$out"
cd "$dir" || { echo "LW1-END rc=2 (cwd) $(date '+%F %T')" >> "$out"; exit 2; }
zsh "$T/📜️fleet-mutex.sh" wasm lw1 -- zsh -c 'echo "LW1-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; s=$(date +%s); nice -n 5 "$@"; rc=$?; echo "LW1-RC rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' lw1 "$@" >> "$out" 2>&1
rc=$?
echo "LW1-END rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
