#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
for f in din4108 din16798 din18599 iso16757 vdi3805 en1990 en1991 en1992 en1994 en1996 en1998 en1999 en1995 en1993; do
  d=$(ls "$N" | while read x; do [ "$(echo "$x" | sed 's/[^a-z0-9]//g')" = "$f" ] && echo "$x"; done)
  "$T/schema-crates-sweep.sh" "$d:$f"
done
