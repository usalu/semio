#!/bin/sh
# 🧪 Laws of batch `cisrc` after apply: `sh verify.sh <MIRROR root with the batch applied>` (mirror from mirror-up.sh; never the live root).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
tools=$(cd "$here/../.." && pwd)
root=${1:?mirror root}
case "$root" in */sm/*) ;; *) echo "refusing: $root is not a mirror" >&2; exit 2;; esac
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "batch is not fully applied in $root" >&2; exit 1; } || true
test_file="./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/🟦️.ts"
real=$(cd "$root" && pwd -P)
cd "$root"
bun test "$test_file"
trace="$real.cisource-trace"
rm -f "$trace"
CISOURCE_TRACE_OUT="$trace" CISOURCE_TRACE_ROOT="$real/" bun test --preload "$tools/trace-reads.ts" -t '^(?!.*declares every workflow and manifest file)' "$test_file"
python3 -I "$tools/check-observed.py" "$real" "$trace"
rm -f "$trace"
python3 -I "$tools/negative.py" "$root"
python3 -I "$tools/../xplat5-tools/tests/test_stage_mode.py" "$here/stage.py"
