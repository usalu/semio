#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
L="$T/🗑️generated/lreg"; mkdir -p "$L"
n="$1"; shift
s=$(date +%s)
timeout ${LREG_TIMEOUT:-7200} repo run-many --exclude-dependencies --parallel=2 --output=stream "$@" > "$L/many-$n.log" 2>&1
echo "many-$n exit=$? secs=$(( $(date +%s)-s ))" | tee -a "$L/summary.txt"
