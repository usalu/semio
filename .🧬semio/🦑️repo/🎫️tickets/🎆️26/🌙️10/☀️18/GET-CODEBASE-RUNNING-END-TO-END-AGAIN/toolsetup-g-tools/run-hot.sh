#!/bin/sh
# usage: run-hot.sh <label> [target]   one official run with the phase profile on (stderr), wall + exit appended; output under 🗑️generated/toolsetup-g
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/toolsetup-g"; mkdir -p "$O"; cd /Users/ueli/Documents/semio || exit 1
start=$(date +%s); SEMIO_REPO_PROFILE=1 nice -n 5 bun repo --tool=bazel run "${2:-@teaching/architecture-course:build}" > "$O/$1.log" 2>&1
echo "exit=$? wall=$(( $(date +%s) - start ))s" >> "$O/$1.log"; tail -1 "$O/$1.log"
