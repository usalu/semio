#!/usr/bin/env bash
# usage: compile-review-w/run.sh <code> <scanner>...   (s:name or t:name) niced, ulimit, one at a time; output in 🗑️generated/compile-review-w
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
O="$T/🗑️generated/compile-review-w"; PY=/home/deck/semio/.venv/bin/python
c=$1; shift
for x in "$@"; do
  d=${x%%:*}; s=${x#*:}; dir="$T/🧰️scripts/compile-review-$d"
  ( ulimit -v 3500000; timeout 100 nice -n 19 $PY -I "$dir/scan_$s.py" "$c" > "$O/$s-$c.txt" 2> "$O/$s-$c.err" )
  echo "$s $c: $(wc -l < "$O/$s-$c.txt") lines, $(wc -c < "$O/$s-$c.err") err"
done
