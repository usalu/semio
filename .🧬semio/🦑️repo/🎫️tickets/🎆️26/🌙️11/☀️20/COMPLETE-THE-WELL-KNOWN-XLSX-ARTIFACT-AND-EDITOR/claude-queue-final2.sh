#!/bin/zsh
# [DEBUG] waits for the running final native run to finish, then reruns it under the guard
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; cd $T
while pgrep -f "run-paged-and-io-laws.sh" >/dev/null; do sleep 30; done
mv 🗑️generated/claude-suites/final-done 🗑️generated/claude-suites/final-done.1 2>/dev/null
./claude-resource-guard.sh ./run-paged-and-io-laws.sh
