#!/bin/sh
# usage: cargo-check.sh <crate> : a real `bun repo cargo ... check -p <crate>` (nice 10), prints rc and wall seconds
cd /Users/ueli/Documents/semio || exit 1
unset SEMIO_SCRIPT_STALL_MS
start=$(date +%s)
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p "$1"
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - start ))s"
