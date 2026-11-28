#!/bin/zsh
# [DEBUG] runs the real-runtime MediaIn laws under the host guard
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-media-in
mkdir -p $O
MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-media-in-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib media_in_of_a_small_pdf_reaches_its_mount > $O/test.log 2>&1; echo "exit $?" > $O/status.log
