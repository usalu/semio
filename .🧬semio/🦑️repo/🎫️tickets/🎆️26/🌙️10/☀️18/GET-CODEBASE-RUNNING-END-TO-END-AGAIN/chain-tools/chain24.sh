#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
G="$1"
print -r -- "=== $(date +%H:%M:%S) START setup bazel"
bun repo tools setup bazel > "$G/setup-bazel.log" 2>&1; rc=$?
print -r -- "=== $(date +%H:%M:%S) END rc=$rc setup bazel"
[ $rc = 0 ] && print "=== CHAIN24 GREEN" || { print "=== SETUP BAZEL FAILED"; tail -30 "$G/setup-bazel.log"; exit 5; }
