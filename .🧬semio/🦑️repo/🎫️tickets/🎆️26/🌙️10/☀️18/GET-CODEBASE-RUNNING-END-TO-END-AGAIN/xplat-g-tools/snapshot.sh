#!/bin/sh
# Writes the staged patch of the overlay edits (overlay = scratchpad clone of the build library, base = its pristine copy).
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd "$SP" || exit 1
diff -ruN --exclude=node_modules ov-base ov > "$C/xplat-g-tools/staged/xplat-g.patch"
grep -c '^diff ' "$C/xplat-g-tools/staged/xplat-g.patch"
