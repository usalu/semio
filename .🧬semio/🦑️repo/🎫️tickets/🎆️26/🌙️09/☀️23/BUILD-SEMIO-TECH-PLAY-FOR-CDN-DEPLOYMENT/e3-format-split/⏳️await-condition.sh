#!/bin/bash
# ⏳️ Waits until a shell condition holds or <seconds> pass, polling every 20 s; prints which happened.
# usage: ⏳️await-condition.sh <seconds> '<condition>'
LIMIT=$1; COND=$2; start=$(date +%s)
while ! bash -c "$COND"; do
  if [ $(( $(date +%s) - start )) -ge "$LIMIT" ]; then echo "timeout after ${LIMIT}s"; exit 1; fi
  sleep 20
done
echo "condition met after $(( $(date +%s) - start ))s"
