#!/bin/sh
# usage: sh regress-extra.sh MIRROR OUT   runs the suites listed in suites-extra.txt
HERE=$(cd "$(dirname "$0")" && pwd)
M=$1; OUT=$2; : > "$OUT"
while IFS= read -r suite; do
  [ -n "$suite" ] && sh "$HERE/regress.sh" "$M" "$OUT" "$suite"
done < "$HERE/suites-extra.txt"
echo done > "$OUT.done"
