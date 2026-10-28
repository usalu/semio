#!/bin/sh
# usage: run-list.sh <list> <out>; runs each test file singly (graph-free), one line per file: rc, pass/fail/skip counts
cd /Users/ueli/Documents/semio || exit 1
: > "$2"
while IFS= read -r f; do
  out=$(nice -n 10 bun test "./$f" 2>&1)
  rc=$?
  counts=$(printf '%s\n' "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')
  printf '%s\trc=%s\t%s\n' "$f" "$rc" "$counts" >> "$2"
  printf '%s\n' "$out" | /usr/bin/grep -E '^\(fail\)|error:' | head -5 | cut -c1-300 >> "$2.detail"
done < "$1"
echo DONE >> "$2"
