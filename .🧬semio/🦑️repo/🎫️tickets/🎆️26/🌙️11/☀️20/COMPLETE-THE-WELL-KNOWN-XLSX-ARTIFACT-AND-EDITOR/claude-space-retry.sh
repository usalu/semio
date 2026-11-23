#!/bin/zsh
# [DEBUG] retries the check while foreign [DEBUG] edits break framework crates
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"
for t in 1 2 3 4 5 6; do
  "$T/claude-space-migrate.sh" "$@"
  tail -1 "$T/🗑️generated/claude-space-migrate/check.log" | grep -q "exit 0" && break
  grep -q "DEBUG" "$T/🗑️generated/claude-space-migrate/check.log" || break
  sleep 60
done
touch "$T/🗑️generated/claude-space-migrate/retry.done"
