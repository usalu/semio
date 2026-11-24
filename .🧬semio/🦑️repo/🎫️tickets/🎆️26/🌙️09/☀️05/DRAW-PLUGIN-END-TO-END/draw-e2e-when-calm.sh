#!/bin/sh
# 🫧️ Starts one Draw end-to-end run once the host has disk space and normal memory pressure; it never touches the host warden.
set -eu
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
RUN="$1"
calm=0
for attempt in $(seq 1 720); do
  free=$(df -k /System/Volumes/Data | tail -1 | awk '{print $4}')
  pressure=$(sysctl -n kern.memorystatus_vm_pressure_level)
  if [ "$free" -ge 8912896 ] && [ "$pressure" -eq 1 ]; then calm=$((calm + 1)); else calm=0; fi
  if [ "$calm" -ge 4 ]; then
    echo "[DEBUG] start $RUN: $free KiB free, pressure $pressure, $(date)"
    exec sh "$T/draw-e2e-2026-11-23.sh" "$RUN"
  fi
  perl -e 'select(undef,undef,undef,15)'
done
echo "[DEBUG] gave up: $free KiB free, pressure $pressure"
exit 3
