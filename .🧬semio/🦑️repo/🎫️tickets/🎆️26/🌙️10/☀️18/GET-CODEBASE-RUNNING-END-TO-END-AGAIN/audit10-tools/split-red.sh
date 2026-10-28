#!/bin/sh
# 🔴 [audit10] the new split law against the lane's installation module (expect a failure). usage: sh split-red.sh <root> <C>
R="$1"; C="$2"; cd "$R" || exit 1
bun "$C/dashhot-tools/env-run.ts" "$R" "@semio-tech/repo-dashboard-rs:test-source-freshness" "$C/🗑️generated/audit10/art" -- test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🧾️compiled/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)|SyntaxError|not found" | head -6
