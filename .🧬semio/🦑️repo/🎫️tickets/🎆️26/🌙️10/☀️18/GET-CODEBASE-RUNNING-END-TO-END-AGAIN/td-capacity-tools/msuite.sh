#!/bin/sh
# 🧪 [td-capacity] runs one process contract suite through the official suite runner (`📜️script.ts test <id>` beneath an issued envelope, as ver-f-tools/proc-run.ts does) INSIDE mirror $1. usage: msuite.sh <mirror> <suite-id> <out-log>
C="$(cd "$(dirname "$0")/.." && pwd)"; R="${TMPDIR%/}/sm/$1"; T="$R/.tc-run"; mkdir -p "$T"
sed "s#/Users/ueli/Documents/semio#$R#g" "$C/ver-f-tools/proc-run.ts" > "$T/proc-run.ts"
cd "$R" || exit 2
VF_WALL=600 bun "$T/proc-run.ts" suite "$2" > "$3" 2>&1; rc=$?
echo "$1 suite $2 rc=$rc $(grep -E '^ *[0-9]+ (pass|fail)$' "$3" | tr -s ' \n' ' ')"
grep -E '^\(fail\)|script\.suite-files-failed|failed' "$3" | head -8 | cut -c1-240
