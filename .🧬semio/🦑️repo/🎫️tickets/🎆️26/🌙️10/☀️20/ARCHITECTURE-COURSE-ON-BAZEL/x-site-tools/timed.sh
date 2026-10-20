#!/bin/sh
# usage: timed.sh <log> <args to bun repo…>; records wall seconds and exit code
log="$1"; shift
start=$(date +%s)
bun repo "$@" > "$log" 2>&1
code=$?
end=$(date +%s)
echo "[x-site] exit=$code seconds=$((end-start)) args=$*" >> "$log"
