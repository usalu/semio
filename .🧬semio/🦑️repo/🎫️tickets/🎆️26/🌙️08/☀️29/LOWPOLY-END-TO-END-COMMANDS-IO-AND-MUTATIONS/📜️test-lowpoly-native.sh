#!/bin/zsh
# 🧪️ Native lib tests of the lowpoly artifact crate; FILTER narrows (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS).
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/test-lowpoly-native${SUFFIX}.txt"
echo "start $(date -u +%FT%TZ) filter=${FILTER}" > "$LOG"
cargo test -p semio-s-artifact-lowpoly-lowpoly --lib --message-format=short -- ${=FILTER} >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
