#!/bin/zsh
# [DEBUG] keeps the retry loop going until the wasm build is green, holding one guard slot the whole time
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; G=$T/🗑️generated/claude-pane
while true; do
  $T/claude-pane-cargo-retry.sh
  if grep -q "^RETRY-OK" $G/retry.status 2>/dev/null; then exit 0; fi
  sleep 60
done
