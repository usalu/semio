#!/usr/bin/env bash
set -u
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
OUT="$T/🗑️generated/w3c-en1999"
PY=/home/deck/semio/.venv/bin/python
E="$PY -I $T/🧰️scripts/schema-evolve/evolve.py"
cd /home/deck/semio
if [ -s "$T/🗑️generated/build-runner/freeze.txt" ] && grep -q en1999 "$T/🗑️generated/build-runner/freeze.txt"; then echo "freeze names en1999" > "$OUT/status.txt"; exit 3; fi
nice -n 19 $E add-leaf en1999 connections "$T/🧰️scripts/schema-evolve/specs/en1999-connection-force-method.json" --apply > "$OUT/apply.txt" 2>&1; echo "apply rc=$?" >> "$OUT/status.txt"
nice -n 19 $E verify en1999 > "$OUT/verify.txt" 2>&1; echo "verify rc=$?" >> "$OUT/status.txt"
nice -n 19 $E verify set-fields en1999 > "$OUT/set-fields.txt" 2>&1; echo "set-fields rc=$?" >> "$OUT/status.txt"
nice -n 19 $E verify steppers en1999 > "$OUT/steppers.txt" 2>&1; echo "steppers rc=$?" >> "$OUT/status.txt"
"$T/🗑️generated/build-runner/heavy.sh" bash "$T/🗑️generated/law-sweep/run-law.sh" "$OUT/law" > "$OUT/law-run.txt" 2>&1; echo "laws rc=$?" >> "$OUT/status.txt"
echo DONE >> "$OUT/status.txt"
