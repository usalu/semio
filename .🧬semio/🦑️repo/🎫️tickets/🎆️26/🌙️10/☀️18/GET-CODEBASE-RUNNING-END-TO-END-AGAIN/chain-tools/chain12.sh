#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
G="$1"
step bun repo show projects > "$G/show-projects.out" 2> "$G/show-projects.err" || { tail -30 "$G/show-projects.err"; exit 3; }
print "=== SHOW GREEN $(wc -l < "$G/show-projects.out")"
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN12 GREEN"
