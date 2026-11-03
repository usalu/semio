#!/usr/bin/env bash
set -u
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
OUT="$T/🗑️generated/en1999-norm"
until grep -q "^collection rc=" "$OUT/apply-status.txt" 2>/dev/null; do sleep 20; done
grep -q "^collection rc=0" "$OUT/apply-status.txt" || { echo "first applies failed" > "$OUT/apply-status-2.txt"; exit 4; }
if [ -s "$T/🗑️generated/build-runner/freeze.txt" ] && grep -q en1999 "$T/🗑️generated/build-runner/freeze.txt"; then echo "freeze names en1999" > "$OUT/apply-status-2.txt"; exit 3; fi
nice -n 19 /home/deck/semio/.venv/bin/python -I "$T/🧰️scripts/schema-evolve/evolve.py" add-collection en1999 "$T/🧰️scripts/schema-evolve/specs/en1999-fire-insulations.json" --apply > "$OUT/insulation-apply.txt" 2>&1
echo "insulations rc=$?" > "$OUT/apply-status-2.txt"
