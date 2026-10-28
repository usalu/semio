#!/bin/sh
# usage: verify.sh -> graph-free single-file runs of the sweep tests IN THE TREE (after apply.sh) plus the existing workflow law of the CI report; prints pass/FAIL per file
cd /Users/ueli/Documents/semio || exit 1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
fail=0
for f in "$L/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts" "$L/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts" "$L/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/🟦️.ts"; do
  out=$(bun test "./$f" 2>&1); rc=$?
  if [ $rc = 0 ]; then echo "pass $f"; else echo "FAIL $f"; echo "$out" | tail -30; fail=1; fi
done
exit $fail
