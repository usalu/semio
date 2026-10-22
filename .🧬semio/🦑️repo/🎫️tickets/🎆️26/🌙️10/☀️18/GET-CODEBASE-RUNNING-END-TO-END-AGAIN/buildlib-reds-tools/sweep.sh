#!/bin/sh
# usage: sweep.sh <outfile> <file>...   runs each explicit test file once (plain bun test, 300 s cap) and appends "file<TAB>pass<TAB>fail<TAB>skip<TAB>rc"
cd /Users/ueli/Documents/semio || exit 1
out="$1"; shift
for f in "$@"; do
  log=$(nice -n 10 perl -e 'alarm 300; exec @ARGV' -- bun test "./$f" 2>&1)
  rc=$?
  p=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) pass$/\1/p' | tail -1)
  fl=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) fail$/\1/p' | tail -1)
  s=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) skip$/\1/p' | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\n' "$f" "${p:-0}" "${fl:-0}" "${s:-0}" "$rc" >> "$out"
done
echo DONE >> "$out"
