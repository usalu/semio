#!/usr/bin/env bash
export PYTHONIOENCODING=utf-8
S="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
OUT="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/ha2/check"
mkdir -p "$OUT"
cd "$S" || exit 1
for a in laptop mvhr-unit pendant-lamp plate-heat-exchanger room-thermostat shower-head sorption-wheel split-ac-indoor-unit thermostatic-radiator-valve toaster vacuum-cleaner; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$a" > "$OUT/$a.txt" 2>&1
  echo "$a exit=$?" >> "$OUT/summary.txt"
done
echo DONE >> "$OUT/summary.txt"
