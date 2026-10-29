#!/bin/bash
source "$(dirname "$0")/lib.sh"
tag=$1
for b in $ALL; do
  f="$G/extfix-$b-$tag.rc"
  if [ -f "$f" ]; then echo "$b rc=$(cat "$f")"; elif [ -f "$G/extfix-$b-$tag.txt" ]; then echo "$b running"; fi
done
[ -f "$G/extfix-run-$tag.rc" ] && echo ALL-DONE
