#!/bin/bash
# usage: run-all.sh <tag>
source "$(dirname "$0")/lib.sh"
tag=$1
rm -f "$G/extfix-run-$tag.rc"
for b in $ALL; do chk_retry "$b" "$tag"; done
echo done > "$G/extfix-run-$tag.rc"
