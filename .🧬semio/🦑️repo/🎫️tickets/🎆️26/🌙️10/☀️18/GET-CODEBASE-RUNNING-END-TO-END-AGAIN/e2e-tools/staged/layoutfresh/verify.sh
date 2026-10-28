#!/bin/sh
# 📏 verify.sh <root>: parse-check of the staged spec. Proof after landing: `... --project desktop --no-deps --repeat-each 12 --workers 4 --grep "nested grid pans"` with 4 learners on one proctor (the shared stack of the gate).
here=$(cd "$(dirname "$0")" && pwd)
list=$(mktemp); printf '%s\n' "$here/files/home.ts" > "$list"
bun "$here/../../../gate-g-tools/scan.ts" "$list" "$(mktemp -d)"
