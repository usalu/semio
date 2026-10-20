#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
step bun repo show projects > "$1/show-projects.out" 2> "$1/show-projects.err" || exit 3
print "=== SHOW GREEN $(wc -l < "$1/show-projects.out")"
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN8 GREEN"
