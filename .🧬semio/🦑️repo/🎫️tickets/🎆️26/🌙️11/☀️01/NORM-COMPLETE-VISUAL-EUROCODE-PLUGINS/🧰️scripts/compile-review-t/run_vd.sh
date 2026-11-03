#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do ( ulimit -v 3500000; timeout 250 nice -n 19 $PY -I "$N/scan_valuederive.py" "$c" > "$O/valuederive-$c.txt" 2> "$O/valuederive-$c.err" ); echo "vd done $c" >> "$O/progress-vd.txt"; done
