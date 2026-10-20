#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
for i in 1 2 3; do step perl -e 'alarm 20000; exec @ARGV' -- bun repo dependencies sync --all && { print "=== SYNC GREEN $(readlink node_modules)"; exit 0; }; sleep 60; done
exit 2
