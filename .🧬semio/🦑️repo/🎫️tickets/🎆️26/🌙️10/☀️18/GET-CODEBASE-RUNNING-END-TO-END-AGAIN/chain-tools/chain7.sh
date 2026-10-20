#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
sleep 300
ok=0
for i in 1 2 3; do step bun repo dependencies lock --all && { ok=1; break; }; sleep 60; done
[ $ok = 1 ] || exit 1
ok=0
for i in 1 2 3 4; do step perl -e 'alarm 20000; exec @ARGV' -- bun repo dependencies sync --all && { ok=1; break; }; sleep 60; done
[ $ok = 1 ] || exit 2
print "=== SYNC GREEN $(readlink node_modules)"
step bun repo show projects > "$1/show-projects.out" 2> "$1/show-projects.err" || exit 3
print "=== SHOW GREEN $(wc -l < "$1/show-projects.out")"
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN7 GREEN"
