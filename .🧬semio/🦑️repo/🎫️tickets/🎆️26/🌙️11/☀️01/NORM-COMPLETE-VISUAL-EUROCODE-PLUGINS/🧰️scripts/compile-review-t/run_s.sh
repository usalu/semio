#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
S="$T/🧰️scripts/compile-review-s"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do
  for s in resolve imports exprpaths visibility deps paths literals assign callargs strmeth receivers params format structs io_fields sqlite_text diff rowkeys match dupes kebab fieldnames methods lock; do
    ( ulimit -v 3500000; timeout 200 nice -n 19 $PY -I "$S/scan_$s.py" "$c" > "$O/$s-$c.txt" 2> "$O/$s-$c.err" )
  done
  echo "s done $c" >> "$O/progress-s.txt"
done
