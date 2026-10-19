#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
ok=0
for i in 1 2 3; do step bun repo dependencies sync --all && { ok=1; break; }; step bun repo dependencies lock --all; done
[ $ok = 1 ] || exit 2
print "=== SYNC GREEN $(readlink node_modules)"
step bun repo show projects > "$1/show-projects.out" 2> "$1/show-projects.err" || exit 3
print "=== SHOW GREEN"
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN GREEN"
