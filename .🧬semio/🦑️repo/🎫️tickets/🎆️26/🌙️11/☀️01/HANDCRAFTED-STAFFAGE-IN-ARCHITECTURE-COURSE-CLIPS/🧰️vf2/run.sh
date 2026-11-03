#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
O="$T/🗑️generated/vf2"
mkdir -p "$O"
for n in "$@"; do
  m=$(sed -n "${n}p" "$T/🧰️ht/modules.txt" | tr -d '\r')
  bash "$T/m9_tests.sh" "$m" </dev/null > "$O/m$n.log" 2>&1
  echo "$n: $(tail -3 "$O/m$n.log" | tr '\n' ' ')"
done
