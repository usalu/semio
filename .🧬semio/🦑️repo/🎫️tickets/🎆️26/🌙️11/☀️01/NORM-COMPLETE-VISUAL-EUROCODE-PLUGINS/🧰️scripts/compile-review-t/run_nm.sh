#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do ( ulimit -v 3500000; timeout 250 nice -n 19 $PY -I "$N/scan_nestedmut.py" "$c" > "$O/nestedmut-$c.txt" 2> "$O/nestedmut-$c.err" ); echo "nm done $c" >> "$O/progress-nm.txt"; done
