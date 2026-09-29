#!/bin/zsh
# 🧪️ WG11 session 14d: runs one command in the native lane (build-fleet-b, private target under the hub dir, CARGO_INCREMENTAL=0,
# NX_DAEMON=false) and appends QUEUED/START/RC lines to its capture under `.🧬semio/🌐hub/s14-wg11-captures/` (rule 26).
# usage: zsh wg11-lane.sh <capture-name> <cwd> -- <command …>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket
name="$1"; dir="$2"; shift 2; [ "$1" = "--" ] && shift
out="$R/.🧬semio/🌐hub/s14-wg11-captures/$name.txt"
export CARGO_INCREMENTAL=0 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$R/.🧬semio/🌐hub/s14-wg11-target"
echo "WG11-QUEUED $(date '+%F %T') cwd=$dir cmd=$*" >> "$out"
cd "$dir" || { echo "WG11-END rc=2 (cwd) $(date '+%F %T')" >> "$out"; exit 2; }
zsh "$T/📜️fleet-mutex.sh" native wg11 -- zsh -c 'echo "WG11-START $(date "+%F %T") load=$(sysctl -n vm.loadavg)"; s=$(date +%s); nice -n 15 "$@"; rc=$?; echo "WG11-RC rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' wg11 "$@" >> "$out" 2>&1
rc=$?
echo "WG11-END rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
