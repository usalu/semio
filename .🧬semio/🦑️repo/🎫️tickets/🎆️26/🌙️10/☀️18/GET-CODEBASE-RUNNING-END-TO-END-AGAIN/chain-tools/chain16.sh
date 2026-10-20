#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
G="$1"
print -r -- "=== $(date +%H:%M:%S) SHOW START"
step bun repo show projects > "$G/show-projects.out" 2> "$G/show-projects.err" || { print "=== SHOW FAILED"; tail -30 "$G/show-projects.err"; exit 3; }
print "=== SHOW GREEN $(date +%H:%M:%S) $(wc -l < "$G/show-projects.out")"
step bun repo tools setup nx > "$G/setup-nx.log" 2>&1 || { print "=== SETUP NX FAILED"; tail -30 "$G/setup-nx.log"; exit 4; }
print "=== SETUP NX GREEN $(date +%H:%M:%S)"
step bun repo tools setup bazel > "$G/setup-bazel.log" 2>&1 || { print "=== SETUP BAZEL FAILED"; tail -30 "$G/setup-bazel.log"; exit 5; }
print "=== CHAIN16 GREEN $(date +%H:%M:%S)"
