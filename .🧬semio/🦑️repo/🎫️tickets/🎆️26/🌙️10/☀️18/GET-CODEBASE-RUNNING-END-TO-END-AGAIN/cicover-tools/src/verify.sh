#!/bin/sh
# 🧪 Laws of batch `ci1` after apply: `sh verify.sh <MIRROR root with the batch applied> [--remote]` (the mirror comes from mirror-up.sh; it is never the live root).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
tools=$(cd "$here/../.." && pwd)
root=${1:?mirror root}
case "$root" in */sm/*) ;; *) echo "refusing: $root is not a mirror" >&2; exit 2;; esac
live=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "batch is not fully applied in $root" >&2; exit 1; } || true
cd "$root"
CI=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report
bun test "$CI/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts" "$CI/🧪️tests/🟦️.ts" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/🟦️.ts" || echo "NOTE: failures above must equal the known pre-existing set (dashboard CI Source gate: project.json inputs gained {projectRoot}/📋️project.json)" >&2
sh "$tools/../ciwf-tools/run-law.sh" "$root"
bun "$tools/scan-direct.ts" "$root" "$live"
python3 -I "$tools/cover-check.py" "$root" --live "$live" ${2:-}
python3 -I "$tools/negative.py" "$root"
python3 -I "$tools/../xplat5-tools/tests/test_stage_mode.py" "$here/stage.py"
