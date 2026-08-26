#!/usr/bin/env bash
# ⏳️ Polls the os-kernel lib until it compiles again. Every stdio subject host build needs it.
cd /Users/ueli/Documents/semio || exit 1
OUT="$1"
for i in $(seq 1 40); do
  cargo check -p semio-framework-os-kernel --lib > /tmp/oskernel-check.txt 2>&1
  rc=$?
  echo "[oskernel] attempt $i rc=$rc $(date +%T)" >> "$OUT"
  if [ $rc -eq 0 ]; then echo "OSKERNEL-GREEN $(date +%T)" >> "$OUT"; exit 0; fi
  grep -c "^error" /tmp/oskernel-check.txt >> "$OUT"
  sleep 120
done
echo "OSKERNEL-STILL-RED $(date +%T)" >> "$OUT"
