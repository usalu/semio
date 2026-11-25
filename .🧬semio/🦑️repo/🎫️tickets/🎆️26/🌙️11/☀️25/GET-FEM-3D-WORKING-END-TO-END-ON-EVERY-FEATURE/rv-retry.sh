#!/usr/bin/env bash
# 🔁️ Retries rv-law.sh until it passes or fails in a FEM crate; framework crates other agents are mid-edit are retried. Usage: rv-retry.sh <filter> <tag> [target]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"
L="$M/🗑️generated/rv/law-$2.log"
for n in $(seq 1 30); do
  "$M/rv-law.sh" "$1" "$2" $3 > "$M/🗑️generated/rv/out-$2.txt" 2>&1
  grep -q "test result" "$L" && break
  grep "could not compile" "$L" | grep -q "semio-s-artifact-fem" && break
  echo "retry $n" >> "$M/🗑️generated/rv/retries.txt"; sleep 120
done
echo done > "$M/🗑️generated/rv/retry-$2.done"
