#!/usr/bin/env bash
set -u
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
cd "$T"
E="/home/deck/semio/.venv/bin/python -I $T/🧰️scripts/schema-evolve/evolve.py"
OUT="$T/🗑️generated/en1999-norm"
if [ -s "$T/🗑️generated/build-runner/freeze.txt" ] && grep -q en1999 "$T/🗑️generated/build-runner/freeze.txt"; then echo "freeze names en1999"; exit 3; fi
nice -n 19 $E add-leaf en1999 connections "$T/🧰️scripts/schema-evolve/specs/en1999-connection-hole-stagger.json" --apply > "$OUT/leaf-apply.txt" 2>&1; echo "leaf rc=$?" >> "$OUT/apply-status.txt"
nice -n 19 $E add-collection en1999 "$T/🧰️scripts/schema-evolve/specs/en1999-sheet-profiles.json" --apply > "$OUT/coll-apply.txt" 2>&1; echo "collection rc=$?" >> "$OUT/apply-status.txt"
