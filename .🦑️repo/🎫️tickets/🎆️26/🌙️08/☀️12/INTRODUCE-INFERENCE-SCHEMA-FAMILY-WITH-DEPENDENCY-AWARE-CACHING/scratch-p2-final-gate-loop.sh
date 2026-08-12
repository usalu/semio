#!/bin/bash
TICKET="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING"
cd /Users/ueli/Documents/semio
for i in $(seq 1 15); do
  echo "=== final-gate attempt $i at $(date) ==="
  CARGO_TARGET_DIR="$TICKET/🎯️target" cargo nextest run --profile long -p semio-s-plugin-stdio --no-fail-fast > "$TICKET/scratch-p2-final-gate-nextest.txt" 2>&1
  if grep -qE "^Summary|tests run:" "$TICKET/scratch-p2-final-gate-nextest.txt"; then
    echo "SUCCESS (real test run completed) on attempt $i"
    exit 0
  fi
  echo "attempt $i hit compile-time churn, waiting 60s"
  sleep 60
done
echo "GAVE UP after 15 attempts"
exit 1
