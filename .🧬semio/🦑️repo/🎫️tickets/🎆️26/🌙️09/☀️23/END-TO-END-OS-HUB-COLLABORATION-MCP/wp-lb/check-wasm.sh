#!/bin/zsh
# 🧪 LB wasm32-wasip2 fast gate through the fleet wasm mutex, default build-dir (the chain's warm wasm32 units): check-wasm.sh <capture-name> <cargo check args…>
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb/generated/$1.txt"; shift
export CARGO_INCREMENTAL=0
{ echo "QUEUED $(date '+%H:%M:%S') cargo check $* --target wasm32-wasip2"; zsh .tmp-ticket/📜️fleet-mutex.sh wasm lb -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 5 cargo check "$@" --target wasm32-wasip2; echo "EXIT $? $(date "+%H:%M:%S")"' lb "$@"; } > "$capture" 2>&1
