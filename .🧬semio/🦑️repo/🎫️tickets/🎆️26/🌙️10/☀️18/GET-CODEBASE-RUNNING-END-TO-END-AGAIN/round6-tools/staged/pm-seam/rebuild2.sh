#!/bin/sh
# usage: sh rebuild2.sh [R1_DIR] [R2_DIR]   round-1 stack (lock-sync closure + round 1) in R1, then round 2 applied on a copy in R2; proves that the staged round-2 set applies cleanly on top of round 1
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R1=${1:-/private/tmp/pm-seam/r1n}; R2=${2:-/private/tmp/pm-seam/r2n}
cd "$HERE"
PM_ROUND=1 sh rebuild.sh "$R1" | tail -1
rm -rf "$R2"; cp -R "$R1" "$R2"
PM_ROUND=2 nice -n 10 python3 apply.py apply --root "$R2" --write --ignore-running | tail -1
