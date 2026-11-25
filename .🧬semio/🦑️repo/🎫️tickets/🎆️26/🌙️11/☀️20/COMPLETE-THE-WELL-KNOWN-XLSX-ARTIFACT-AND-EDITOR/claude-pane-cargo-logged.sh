#!/bin/bash
# [DEBUG] runs claude-pane-cargo.sh with its own timestamped log (output, signal diagnosis and exit line) under the generated folder
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"
L=$T/🗑️generated/claude-pane/office-rebuild-$(date +%H%M%S).log
echo "[DEBUG] log $L"
echo "[DEBUG] start $(date +%T) disk $(df -g /Users/ueli/Documents/semio | tail -1 | awk '{print $4}') GiB free, memory $(memory_pressure | tail -1)" > $L
$T/claude-pane-cargo.sh >> $L 2>&1; code=$?
echo "[DEBUG] exit $code at $(date +%T) disk $(df -g /Users/ueli/Documents/semio | tail -1 | awk '{print $4}') GiB free, memory $(memory_pressure | tail -1)" >> $L
echo "exit $code"
exit $code
