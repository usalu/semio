#!/usr/bin/env bash
# Usage: p3-run.sh <logname> '<cargo args after cargo>'   one package per builder.sh exec, jobs 3
T="$(cd "$(dirname "$0")" && pwd)"; R="$(cd "$T/../../../../../../.." && pwd)"
L="$T/🗑️generated/p3-$1.log"
cd "$R" && "$T/builder.sh" exec "cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d && cargo $2 --offline --jobs ${P3_JOBS:-3}; echo \"RC:\$?\"" > "$L" 2>&1
