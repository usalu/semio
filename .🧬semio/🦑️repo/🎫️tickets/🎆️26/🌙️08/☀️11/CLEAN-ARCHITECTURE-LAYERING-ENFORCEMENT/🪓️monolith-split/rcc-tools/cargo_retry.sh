#!/bin/zsh
# 🔁 Retries one launcher cargo call while the launcher refuses for capacity or peer-registry reasons. Args as cargo_check.sh.
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
T="$(dirname "$0")"
LOG="$1"
for i in $(seq 1 40); do
  "$T/cargo_check.sh" "$@"
  if /usr/bin/grep -q -E 'Insufficient physical compiler initial headroom|Original selected owner registry changed|ENFILE|Too many open files' "$G/$LOG.log"; then
    echo "attempt $i refused $(date +%H:%M:%S)" >> "$G/$LOG.attempts"
    sleep 60
  else
    echo "attempt $i ran $(date +%H:%M:%S)" >> "$G/$LOG.attempts"
    break
  fi
done
