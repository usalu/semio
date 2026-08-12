#!/bin/bash
TICKET="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING"
cd /Users/ueli/Documents/semio
for i in $(seq 1 20); do
  echo "=== targeted-noincr attempt $i at $(date) ==="
  CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$TICKET/🎯️target" cargo test -p semio-s-plugin-stdio --lib -- inference_default_law collects_headings --test-threads=1 > "$TICKET/scratch-p2-verify-baseline-targeted-noincr.txt" 2>&1
  if grep -qE "test result:" "$TICKET/scratch-p2-verify-baseline-targeted-noincr.txt"; then
    echo "SUCCESS (real test run completed) on attempt $i"
    exit 0
  fi
  echo "attempt $i hit compile-time churn, waiting 60s"
  sleep 60
done
echo "GAVE UP after 20 attempts"
exit 1
