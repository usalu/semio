#!/bin/zsh
# [DEBUG] after the stdio wasm retry loop ends: final native xlsx run, then the paged backtrace run, each under the resource guard
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; cd $T
while pgrep -f "claude-pane-cargo-retry.sh|claude-pane-cargo-forever.sh" >/dev/null; do sleep 60; done
mv 🗑️generated/claude-suites/final-done 🗑️generated/claude-suites/final-done.prev 2>/dev/null
./claude-resource-guard.sh ./run-paged-and-io-laws.sh
zsh ./claude-paged-backtrace.sh
echo "queue done $(date +%H:%M)" > 🗑️generated/claude-suites/queue-native-after.status
