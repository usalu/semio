#!/bin/zsh
# 🔁 Retries one mutex-guarded launcher call every 10 minutes while the compiler headroom gate refuses it. Args as cargo_check.sh.
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
for i in 1 2 3 4 5 6; do
  sleep 600
  "$(dirname "$0")/mutex_cargo.sh" "$@"
  if /usr/bin/grep -q 'Insufficient physical compiler initial headroom' "$G/$1.log"; then echo "attempt $i refused $(date +%H:%M:%S)" >> "$G/$1.attempts"; else echo "attempt $i ran $(date +%H:%M:%S)" >> "$G/$1.attempts"; break; fi
done
