#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
G="$1"
print -r -- "=== $(date +%H:%M:%S) START dependencies lock --scope ."
bun repo dependencies lock --scope . > "$G/deps-lock.log" 2>&1; rc=$?
print -r -- "=== $(date +%H:%M:%S) END rc=$rc dependencies lock"
[ $rc = 0 ] && print "=== CHAIN25 GREEN" || { print "=== DEPS LOCK FAILED"; /usr/bin/grep -a -v '^\[script\] {' "$G/deps-lock.log" | tail -30; exit 5; }
