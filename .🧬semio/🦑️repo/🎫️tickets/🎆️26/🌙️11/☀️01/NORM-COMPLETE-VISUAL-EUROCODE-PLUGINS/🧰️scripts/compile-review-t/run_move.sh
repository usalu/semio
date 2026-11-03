#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
N="$T/🧰️scripts/compile-review-t"; O="$T/🗑️generated/compile-review-t"; PY=/home/deck/semio/.venv/bin/python
for c in "$@"; do ( ulimit -v 3500000; timeout 200 nice -n 19 $PY -I "$N/scan_move.py" "$c" > "$O/move-$c.txt" 2> "$O/move-$c.err" ); for s in traitdupes; do ( ulimit -v 3500000; timeout 200 nice -n 19 $PY -I "$N/scan_$s.py" "$c" > "$O/$s-$c.txt" 2>/dev/null ); done; echo "move done $c" >> "$O/progress-move.txt"; done
