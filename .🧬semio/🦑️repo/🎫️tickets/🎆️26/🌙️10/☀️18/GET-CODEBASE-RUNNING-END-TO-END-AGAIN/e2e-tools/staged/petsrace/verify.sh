#!/bin/sh
# 🪤 verify.sh <root>: parse-check the staged specs (Bun transpile + node --check). Proof of the fix = the focused run on live after the gate landed it:
#   E2E_STAMP=<id> nice -n 10 bash <C>/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev -- --project pets --no-deps --repeat-each 40 --workers 4 --grep "opens its parachute|falling pet passes in front|let go high above"
# (base: 17 of 80 failed at load 40-60; 12 of 12 passed at load 29 with 3 workers, 6 repeats).
here=$(cd "$(dirname "$0")" && pwd)
list=$(mktemp)
printf '%s\n%s\n' "$here/files/walk.ts" "$here/files/kit.ts" > "$list"
bun "$here/../../../gate-g-tools/scan.ts" "$list" "$(mktemp -d)"
