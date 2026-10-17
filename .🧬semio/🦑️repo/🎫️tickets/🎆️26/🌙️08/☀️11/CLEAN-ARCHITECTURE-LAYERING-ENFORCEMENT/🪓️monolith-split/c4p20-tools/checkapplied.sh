#!/bin/zsh
# usage: checkapplied.sh <label>   runs `new crates <remodeling pkg> --check-applied --base=f856dff0813`, retrying while a peer's infra edit makes the script fail to load
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1
log="$F/🗑️generated/c4p20-checkapplied-$label.log"
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30; do
  mkdir -p "/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p20/sbx-ca-$label"
  ( ulimit -n 65536; cd /Users/ueli/Documents/semio; perl -e 'alarm 3300; exec @ARGV' -- bun ./📜️script.ts new crates '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust' --check-applied --base=f856dff0813 --sandbox="/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p20/sbx-ca-$label" > "$log" 2>&1; echo "exit=$?" >> "$log" )
  if /usr/bin/grep -q "Cannot find module\|Original selected owner registry changed" "$log"; then echo "attempt $attempt: infra not ready, retry" >> "$F/🗑️generated/c4p20-checkapplied-$label.attempts"; sleep 60; continue; fi
  break
done
echo "finished label=$label" >> "$F/🗑️generated/c4p20-checkapplied-$label.attempts"
