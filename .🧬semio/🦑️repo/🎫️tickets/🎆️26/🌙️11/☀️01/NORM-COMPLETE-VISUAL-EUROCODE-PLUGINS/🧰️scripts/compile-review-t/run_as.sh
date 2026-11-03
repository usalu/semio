#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do ( ulimit -v 3500000; timeout 250 nice -n 19 $PY -I "$N/scan_assoc.py" "$c" > "$O/assoc-$c.txt" 2> "$O/assoc-$c.err" ); echo "as done $c" >> "$O/progress-as.txt"; done
