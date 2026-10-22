#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
G="$1"
step bun repo tools setup nx > "$G/setup-nx.log" 2>&1 || { print "=== SETUP NX FAILED"; tail -30 "$G/setup-nx.log"; exit 4; }
print "=== SETUP NX GREEN $(date +%H:%M:%S)"
step bun repo tools setup bazel > "$G/setup-bazel.log" 2>&1 || { print "=== SETUP BAZEL FAILED"; tail -30 "$G/setup-bazel.log"; exit 5; }
print "=== CHAIN23 GREEN $(date +%H:%M:%S)"
