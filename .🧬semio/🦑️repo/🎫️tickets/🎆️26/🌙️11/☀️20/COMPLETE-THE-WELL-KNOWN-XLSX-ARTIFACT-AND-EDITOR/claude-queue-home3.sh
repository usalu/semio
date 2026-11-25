#!/bin/zsh
# [DEBUG] queued Home workspace test run (one heavy job at a time), then deletes its build dirs
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; cd $T
SLOTS=1 ./claude-resource-guard.sh env WS=home CMD=test LOG=home3 SEL="--workspace" ./claude-space-tests2.sh --no-fail-fast
echo "home3 exit $?" > 🗑️generated/claude-space-tests2/home3.status
rm -rf 🗑️generated/claude-space-tests2/target 🗑️generated/claude-space-tests2/build
