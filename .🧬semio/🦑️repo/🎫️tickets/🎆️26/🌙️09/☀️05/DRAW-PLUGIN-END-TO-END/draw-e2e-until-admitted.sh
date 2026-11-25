#!/bin/sh
# 🔁️ Repeats one Draw end-to-end run while a crate another agent is editing exceeds its source boundary; every other outcome ends the loop.
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
G="$T/🗑️generated"
for attempt in $(seq 1 60); do
  : > "$G/draw-native-component-$1.log"
  SEMIO_OFFLOAD=never sh "$T/draw-e2e-when-calm.sh" "$1" > "$G/e2e-$1.log" 2>&1
  grep -aq "Source boundary exceeded\|storage refused" "$G/draw-native-component-$1.log" || exit 0
  echo "[DEBUG] attempt $attempt refused before compiling $(date +%T)" >> "$G/e2e-$1.attempts"
  perl -e 'select(undef,undef,undef,180)'
done
