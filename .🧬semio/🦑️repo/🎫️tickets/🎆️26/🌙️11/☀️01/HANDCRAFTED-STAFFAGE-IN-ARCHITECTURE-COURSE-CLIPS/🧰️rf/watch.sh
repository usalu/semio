#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
OUT="$T/🗑️generated/rf/watch"
mkdir -p "$OUT"
for i in $(seq 1 80); do
  for f in $(find .🧬semio/🎓️teaching/⚡️cache/tests -name 'run.*' -mmin -8 2>/dev/null); do
    cp "$f" "$OUT/$(echo "$f" | md5sum | cut -c1-8)-$(basename "$f")" 2>/dev/null
  done
  sleep 4
done
