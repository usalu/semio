#!/usr/bin/env bash
# usage: run_all.sh <code>...   runs the S scanners plus the T scanners per code, sequentially
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
S="$T/🧰️scripts/compile-review-s"; N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"
PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do
  for s in resolve imports exprpaths visibility deps paths literals assign callargs strmeth receivers params format structs io_fields sqlite_text diff rowkeys match dupes kebab fieldnames methods lock; do
    timeout 150 $PY -I "$S/scan_$s.py" "$c" > "$O/$s-$c.txt" 2> "$O/$s-$c.err"
  done
  for s in twophase tempborrow cmp derive arity; do
    timeout 150 $PY -I "$N/scan_$s.py" "$c" > "$O/$s-$c.txt" 2> "$O/$s-$c.err"
  done
  echo "done $c" >> "$O/progress.txt"
done
