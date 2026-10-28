#!/bin/sh
# 🧪 [td-capacity] runs one process contract suite through the official suite runner (`📜️script.ts test <id>` beneath an issued envelope, the same birth as `repo run @semio-tech/framework-process:test-testing`; ver-f-tools/proc-run.ts) with <root> as the checkout. usage: suite-in-root.sh <root> <suite-id> [out-log]; prints `<suite> rc=<n> <pass/fail lines>` and the failing tests; exit code = the suite's.
C="$(cd "$(dirname "$0")/.." && pwd)"; R="$(cd -P "$1" && pwd -P)"; T="$R/.tc-run"; mkdir -p "$T"
OUT=${3:-$T/suite-$2.log}
sed "s#/Users/ueli/Documents/semio#$R#g" "$C/ver-f-tools/proc-run.ts" > "$T/proc-run.ts"
cd "$R" || exit 2
VF_WALL=600 bun "$T/proc-run.ts" suite "$2" > "$OUT" 2>&1; rc=$?
echo "$2 rc=$rc $(grep -E '^ *[0-9]+ (pass|fail)$' "$OUT" | tr -s ' \n' ' ')"
grep -E '^\(fail\)' "$OUT" | sort -u | cut -c1-240
exit $rc
