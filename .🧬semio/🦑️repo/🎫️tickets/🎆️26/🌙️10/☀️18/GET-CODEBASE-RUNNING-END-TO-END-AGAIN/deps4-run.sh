#!/bin/sh
# usage: deps4-run.sh <log> <alarm> <bun repo args...>
cd /Users/ueli/Documents/semio || exit 1
log="$1"; alarm="$2"; shift 2
perl -e 'alarm shift; exec @ARGV' "$alarm" "$@" > "$log" 2>&1 </dev/null
echo "rc=$?" >> "$log"
