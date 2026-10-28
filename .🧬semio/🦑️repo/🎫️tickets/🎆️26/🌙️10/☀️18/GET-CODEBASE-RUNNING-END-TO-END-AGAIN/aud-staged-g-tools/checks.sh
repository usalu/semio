#!/bin/bash
# [aud-staged-g] read-only check modes of staged gate-g batches (no apply, no build)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/aud-staged-g-tools/out"
run() { name=$1; shift; { "$@" 2>&1; echo "EXIT $?"; } > "$O/$name.txt"; echo "$name: $(tail -1 "$O/$name.txt")"; }
run nxfix-core python3 "$C/nxfix-g-tools/stage.py" check --batch core
run nxfix-nxpatch python3 "$C/nxfix-g-tools/stage.py" check --batch nx-patch
run nxlock python3 "$C/nxlock-g-tools/stage.py" check
run obsrows python3 "$C/obsrows-g-tools/stage.py" check
run setup python3 "$C/setup-g-tools/staged/stage.py" check
run rev3fix python3 "$C/rev3fix-g-tools/batch.py" "$C/rev3fix-g-tools/staged/rev3fix" check
run store-foldsafe python3 "$C/store-g-tools/staged/apply-foldsafe-preflight.py" --check
run store-managed python3 "$C/store-g-tools/staged/apply-managed-parent-guard.py" --check
run laws-build-tools python3 "$C/laws-g-tools/apply-build-tools.py" --check
run laws-route-unrouted python3 "$C/laws-g-tools/route-unrouted.py" --check
run laws-dedupe python3 "$C/laws-g-tools/dedupe-tables.py" --check
run laws-oracle python3 "$C/laws-g-tools/oracle-required.py" --dry
run setup-overlay python3 "$C/setup-g-tools/staged/build-overlay.py" --check
