#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; M="$C/staged-merge-tools"
step() { print -r -- "=== $(date +%H:%M:%S) START $*"; "$@"; local rc=$?; print -r -- "=== $(date +%H:%M:%S) END rc=$rc $*"; return $rc; }
step sh "$C/stage-fix-tools/restage-all.sh" || exit 1
step bun "$M/plan.ts" plan || exit 2
step sh "$M/apply-all.sh" --dry-run --ignore-sync --include-provisional || exit 3
step sh "$C/deps5-tools/apply.sh" check || exit 4
print "=== APPLY-PREP READY"
