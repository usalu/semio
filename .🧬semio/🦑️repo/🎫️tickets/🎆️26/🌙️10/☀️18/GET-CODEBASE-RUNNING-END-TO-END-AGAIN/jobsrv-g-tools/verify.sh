#!/bin/sh
# usage: verify.sh [tree|overlay <dir>] -> graph-free single-file runs of the jobsrv-g laws and of the unit-cache suites they touch (default: IN THE TREE, after `stage.py apply`); prints pass/FAIL per file, exit 1 on any failure
cd "${2:-/Users/ueli/Documents/semio}" || exit 1
U="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
fail=0
for f in "$U/🎰️jobserver/🧪️tests/🟦️.ts" "$U/🎰️jobserver/🧪️tests/🚦️pool/🟦️.ts" "$U/🧪️tests/🟦️.ts" "$U/🧪️tests/🪟️windows/🟦️.ts"; do
  out=$(bun test "./$f" 2>&1); rc=$?
  if [ $rc = 0 ]; then echo "pass $(echo "$out" | grep -E '^ *[0-9]+ pass' | tr -s ' ') $f"; else echo "FAIL $f"; echo "$out" | tail -40; fail=1; fi
done
out=$(bun build --target=bun "./$U/🏁️entry.ts" 2>&1 >/dev/null); if [ $? = 0 ]; then echo "pass native entry bundles"; else echo "FAIL native entry bundle"; echo "$out" | tail; fail=1; fi
exit $fail
