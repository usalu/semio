#!/bin/sh
# 🛂️ Laws of batch `acceptance` after apply: `sh verify.sh <MIRROR root with the batch applied>` (never the live root). The mirror holds the library, the process module, the launcher and `.github` only:
# the two tests listed in expected-failures.txt need trees a mirror does not copy (they fail the same way on the base files, measured), every other test of the suites must pass.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?mirror root}
case "$root" in */sm/*) ;; *) echo "refusing: $root is not a mirror" >&2; exit 2;; esac
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "batch is not fully applied in $root" >&2; exit 1; } || true
cd "$root"
for suite in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🛂️acceptance/🧪️tests/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts"; do
  echo "== $suite"
  out=$(bun test "./$suite" 2>&1 || true)
  echo "$out" | grep -E '^ *[0-9]+ (pass|fail)'
  fails=$(echo "$out" | sed -n 's/^(fail) \(.*\) \[[0-9.]*ms\]$/\1/p' | sort)
  want=$(grep -F "$suite|" "$here/expected-failures.txt" | sed 's/^[^|]*|//' | sort || true)
  if [ "$fails" != "$want" ]; then echo "UNEXPECTED FAILURE SET in $suite:" >&2; echo "$fails" >&2; exit 1; fi
done
python3 -I "$here/../../../xplat5-tools/tests/test_stage_mode.py" "$here/stage.py"
echo "acceptance VERIFIED"
