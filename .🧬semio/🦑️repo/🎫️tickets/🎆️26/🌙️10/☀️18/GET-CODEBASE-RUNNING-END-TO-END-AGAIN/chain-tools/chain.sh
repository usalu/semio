#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
step bun repo dependencies lock --all || exit 1
for i in 1 2 3 4; do step bun repo dependencies sync --all && break; done || exit 2
step bun repo show projects > "$1/show-projects.out" || exit 3
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN GREEN"
