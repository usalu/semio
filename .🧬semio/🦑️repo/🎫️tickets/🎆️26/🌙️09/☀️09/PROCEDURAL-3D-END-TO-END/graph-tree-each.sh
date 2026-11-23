#!/bin/sh
# Runs one test file per process: prints "PASS|FAIL file".
G="$(dirname "$0")/🗑️generated"; f="$1"
if timeout 150 repo tools exec --light bun -- test "$f" > "$G/each-$(echo "$f" | md5).txt" 2>&1; then echo "PASS $f"; else echo "FAIL $f $G/each-$(echo "$f" | md5).txt"; fi
