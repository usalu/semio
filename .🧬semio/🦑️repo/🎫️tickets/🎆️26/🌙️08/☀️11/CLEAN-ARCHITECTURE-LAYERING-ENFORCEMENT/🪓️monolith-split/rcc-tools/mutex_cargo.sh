#!/bin/zsh
# 🔒️ Runs one batched launcher call under the fleet mkdir mutex of the Wave 5 brief and always releases it. Args as cargo_check.sh.
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
L="$G/cargo-mutex.lock"
until mkdir "$L" 2>/dev/null; do
  if [ -f "$L/owner" ] && [ -n "$(find "$L/owner" -mmin +75 2>/dev/null)" ]; then echo "stale lock removed $(cat $L/owner)" >> "$G/rcc-mutex.log"; rm -f "$L/owner"; rmdir "$L" 2>/dev/null; continue; fi
  sleep 60
done
echo "rcc $$ $(date)" > "$L/owner"
echo "acquired $(date +%H:%M:%S)" >> "$G/rcc-mutex.log"
"$(dirname "$0")/cargo_check.sh" "$@"
rm -f "$L/owner"; rmdir "$L"
echo "released $(date +%H:%M:%S)" >> "$G/rcc-mutex.log"
