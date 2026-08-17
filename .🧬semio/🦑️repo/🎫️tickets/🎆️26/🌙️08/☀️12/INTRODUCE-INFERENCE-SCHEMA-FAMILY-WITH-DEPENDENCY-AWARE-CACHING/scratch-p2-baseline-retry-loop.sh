#!/bin/bash
TICKET="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING"
cd /Users/ueli/Documents/semio
for i in $(seq 1 12); do
  echo "=== attempt $i at $(date) ==="
  CARGO_TARGET_DIR="$TICKET/🎯️target" cargo nextest run --profile long -p semio-s-plugin-stdio --no-fail-fast > "$TICKET/scratch-p2-baseline-nextest.txt" 2>&1
  if ! grep -q "error: couldn't read" "$TICKET/scratch-p2-baseline-nextest.txt"; then
    echo "SUCCESS on attempt $i"
    exit 0
  fi
  echo "attempt $i hit glue.rs churn, waiting 45s"
  sleep 45
done
echo "GAVE UP after 12 attempts"
exit 1
