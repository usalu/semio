#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
until command grep -qE "^(DONE|STOP)" "$C/🗑️generated/deps4-loop.log" 2>/dev/null && ! pgrep -f "bun repo dependencies sync" >/dev/null; do sleep 30; done
command grep -q "^DONE" "$C/🗑️generated/deps4-loop.log" || { print "=== loop STOP, not green"; exit 2; }
print "=== $(date +%H:%M:%S) SYNC GREEN node_modules=$(readlink node_modules)"
step bun repo show projects > "$1/show-projects.out" 2> "$1/show-projects.err" || exit 3
print "=== SHOW GREEN"
step bun repo tools setup nx || exit 4
step bun repo tools setup bazel || exit 5
print "=== CHAIN6 GREEN"
