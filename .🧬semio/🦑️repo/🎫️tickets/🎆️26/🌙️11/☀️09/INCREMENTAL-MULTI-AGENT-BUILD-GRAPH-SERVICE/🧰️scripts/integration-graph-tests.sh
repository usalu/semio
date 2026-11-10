#!/usr/bin/env bash
# usage: integration-graph-tests.sh OUT LIST ; runs every test file named in LIST (first field of baseline lines) in light mode
cd /home/deck/semio
OUT="$1"; : > "$OUT"
cut -d' ' -f1 "$2" | while read -r f; do
  r=$(repo tools exec --light bun -- test "./$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)|error:" | tr '\n' ' ')
  echo "$f => $r" >> "$OUT"
done
