#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do for s in twophase tempborrow cmp derive arity; do
  ( ulimit -v 3500000; timeout 200 nice -n 19 $PY -I "$N/scan_$s.py" "$c" > "$O/$s-$c.txt" 2> "$O/$s-$c.err" ); done; echo "new done $c" >> "$O/progress-new.txt"; done
