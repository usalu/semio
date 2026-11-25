#!/bin/bash
# [DEBUG] rebuilds stdio + stdio-office wasm (natural-file single-turn decode fix) and strips name sections; logs under the ticket generated folder
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"
L=$T/🗑️generated/claude-pane/office-rebuild.log
echo "$(date +%H:%M:%S) start" >> $T/🗑️generated/claude-pane/lanes.status
$T/claude-pane-cargo.sh > $L 2>&1
echo "$(date +%H:%M:%S) office rebuild cargo $(tail -1 $L)" >> $T/🗑️generated/claude-pane/lanes.status
