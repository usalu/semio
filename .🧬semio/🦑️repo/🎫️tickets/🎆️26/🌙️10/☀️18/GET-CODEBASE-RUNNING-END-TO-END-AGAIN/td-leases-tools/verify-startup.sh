#!/bin/sh
# 🚦 [td-leases] verify batch `startup` on <root> (a mirror, or live after apply): the door startup law (every row, incl. the leases/capacity/doctor/caller entries) and the suites of every touched importer pass. usage: sh verify.sh <root>
R="${1:?root}"; B="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; rc=0
for d in "$B/🚪️door/🧪️tests/🚦️startup" "$B/🚪️door/🧪️tests" "$B/🔒️leases/⌨️command/🧪️tests" "$B/🔒️leases/⌨️command/🧪️tests/🧮️status" "$B/🩺️doctor/🧪️tests" "$B/🩺️doctor/🧪️tests/🔎️probes" "$B/🩺️doctor/🧪️tests/⌨️command" "$B/🌡️host-capacity/🧪️tests" "$B/🌡️host-capacity/🧪️tests/🧮️arguments" "$B/🌡️host-capacity/🧪️tests/📈️memory-fit" "$B/🌡️host-capacity/📖️texts/🧪️tests" "$B/🪪️agent-sessions/⌨️command/🧪️tests" "$B/🧰️tool-registry/🧪️tests/🔎️exec-command" "$R/🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests" "$R/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs/🧪️tests"; do
 out=$(cd "$d" && bun test --timeout 170000 ./🟦️.ts 2>&1); s=$?
 sum=$(printf '%s\n' "$out" | grep -E '^ *[0-9]+ (pass|fail)$' | tail -2 | tr -s ' \n' ' ')
 echo "${d#$R/}:$sum exit $s" | sed 's#🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/##'; [ $s -eq 0 ] || { rc=1; printf '%s\n' "$out" | grep -E '^\(fail\) ' | tail -5; }
done
[ $rc -eq 0 ] && echo "VERIFY OK startup" || echo "VERIFY FAILED startup"; exit $rc
