#!/bin/bash
# usage: run.sh <tag> <batch>...
source "$(dirname "$0")/lib.sh"
tag=$1; shift
for b in "$@"; do chk "$b" "$tag"; done
echo done > "$G/extfix-run-$tag.rc"
