#!/bin/zsh
# [DEBUG] waits for the queued cleanup-pump test (its guard process) to finish, then runs the guarded stdio then stdio-office rebuild
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"
until pgrep -f "claude-resource-guard.sh ./claude-cleanup-pump-test.sh" >/dev/null; do sleep 15; done
echo "[DEBUG] $(date +%T) pump test guard seen"
while pgrep -f "claude-resource-guard.sh ./claude-cleanup-pump-test.sh" >/dev/null; do sleep 30; done
echo "[DEBUG] $(date +%T) pump test finished; rebuilding"
MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-pane-cargo-logged.sh; echo "rebuild exit $?"
