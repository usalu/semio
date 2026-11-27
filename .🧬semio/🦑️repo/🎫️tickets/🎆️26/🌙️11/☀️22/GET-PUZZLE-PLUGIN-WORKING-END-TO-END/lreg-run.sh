#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
L="$T/🗑️generated/lreg"; mkdir -p "$L"
for t in "$@"; do
  f="$L/$(echo "$t" | tr '/:@' '___').log"
  s=$(date +%s)
  timeout ${LREG_TIMEOUT:-1500} repo run "$t" ${LREG_FLAGS---exclude-dependencies} > "$f" 2>&1
  echo "$t exit=$? secs=$(( $(date +%s)-s ))" | tee -a "$L/summary.txt"
done
