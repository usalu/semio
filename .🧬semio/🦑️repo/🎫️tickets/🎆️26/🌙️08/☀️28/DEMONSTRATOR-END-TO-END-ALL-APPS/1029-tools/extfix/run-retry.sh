#!/bin/bash
# usage: run-retry.sh <tag> <batch>...   (ENOSPC / loan-deadline are infrastructure: retried)
source "$(dirname "$0")/lib.sh"
tag=$1; shift
rm -f "$G/extfix-run-$tag.rc"
for b in "$@"; do chk_retry "$b" "$tag"; done
echo done > "$G/extfix-run-$tag.rc"
