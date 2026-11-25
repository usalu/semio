#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
until grep -q "^en1993 exit" "$G/sweep-status.log" 2>/dev/null; do sleep 20; done
for round in 1 2 3; do
  for f in din4108 din16798 din18599 iso16757 en1991 en1992 en1994 en1996 en1998 en1999 en1995 en1993; do
    last=$(grep "^$f exit" "$G/sweep-status.log" | tail -1)
    case "$last" in "$f exit 0") continue;; esac
    case "$last" in "$f exit 101") grep -q "test result: FAILED" "$G/sweep-$f.log" && continue;; esac
    d=$(ls "$N" | while read x; do [ "$(echo "$x" | sed 's/[^a-z0-9]//g')" = "$f" ] && echo "$x"; done)
    "$T/schema-crates-sweep.sh" "$d:$f"
  done
done
echo RETRYDONE >> "$G/sweep-status.log"
