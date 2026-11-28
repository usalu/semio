#!/bin/zsh
# [DEBUG] runs the real-runtime MediaIn laws under the host guard
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-media-in-xlsx
mkdir -p $O
MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-media-in-xlsx-cargo.sh test -p semio-s-artifact-stdio-xlsx-transitional --lib media_in_of_ -- --nocapture > $O/test.log 2>&1; echo "exit $?" > $O/status.log
