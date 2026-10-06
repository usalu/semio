#!/bin/zsh
# 🔁️ Cross-plugin acceptance batch: runs every "Remaining families" command of `📓️s5-agnostic-matrix.md` (one
# `🧪️s5-agnostic-run-family.sh <family> <crate>…` per line) one after the other, detached, with no agent attached. A family that
# answers exit 5 (activation flag present or fewer than 12 GiB free) or exit 4 (disk fell below 8 GiB mid-run) is re-issued after
# three minutes, at most 200 times (ten hours); any other exit moves on. One line per family in `🗑️generated/s5-agnostic/batch.events`; the
# runner itself appends the per-crate rows to `acceptance-results.tsv`. The matrix is refreshed after every family.
setopt no_bg_nice
ticket="${0:A:h}"
out="$ticket/🗑️generated/s5-agnostic"
mkdir -p "$out"
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
/usr/bin/grep -E '^zsh \$T/🧪️s5-agnostic-run-family\.sh ' "$ticket/📓️s5-agnostic-matrix.md" > "$out/batch.commands"
while IFS= read -r line; do
  family="${${(z)line}[3]}"
  tries=0
  while true; do
    eval "$line" > "$out/batch-$family.out" 2>&1
    code=$?
    if { [ $code -eq 5 ] || [ $code -eq 4 ]; } && [ $tries -lt 200 ]; then tries=$((tries + 1)); sleep 180; continue; fi
    break
  done
  print -r -- "$(date '+%F %T') $family exit=$code tries=$tries free=$(df -g /System/Volumes/Data | awk 'NR>1{print $4}')GiB" >> "$out/batch.events"
  python3 "$ticket/🧪️s5-agnostic-matrix.py" > /dev/null 2>&1
done < "$out/batch.commands"
print -r -- "$(date '+%F %T') BATCH DONE" >> "$out/batch.events"
