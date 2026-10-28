#!/bin/sh
# rebase-g: compose2.sh <overlay> <mine-last|mine-first|mine-only>: rev3fix on the LIVE tree (rev3-g win-withheld + upgrade-before-custody are landed) with gram2-g/index-chunks in either order, on a short-root overlay of the touched directories (never the live tree). Then: VERIFY_ROOT=<overlay> sh staged/rev3fix/verify.sh
set -e
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; OV=$1; ORDER=$2
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; P="🧰️framework/🔨️modules/🏃️process"
rm -rf "$OV"
python3 -I "$C/rev3fix-g-tools/overlay.py" "$OV" @"$C/rev3fix-g-tools/staged/rev3fix/edits.json" @"$C/gram2-g-tools/staged/index-chunks/edits.json" "$L" "$P" 2>&1 | tail -2
mine() { python3 -I "$C/rev3fix-g-tools/batch.py" "$C/rev3fix-g-tools/staged/rev3fix" apply "$OV" --accept-drift | grep -v "^before\|^absent\|^after\|^drift"; }
chunks() { BATCH_ROOT="$OV" python3 -I "$C/gram2-g-tools/staged/batch_generic.py" "$C/gram2-g-tools/staged/index-chunks" apply | tail -3; }
case "$ORDER" in mine-only) mine;; mine-first) mine; chunks;; *) chunks; mine;; esac
