#!/bin/sh
# 🤝 Laws of batch `quietfc-mutual` after apply: sh verify.sh <repository or mirror root>; exit 1 on any failure except four real-process laws (gate lending, three handover cases) that fail only under the mirror seal, identically on the base.
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?root}
python3 -I "$here/stage.py" check --root "$root" || exit 1
HC="$root/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for o in 🔼️upgrade 🌤️followup 🪜️phases; do (cd "$HC/🧪️tests/$o/🐍️oracle" && python3 ./🐍️.py --check) || exit 1; done
out=$(mktemp)
(cd "$HC" && for f in 🔼️upgrade/🤝️mutual 🌤️followup 🔼️upgrade 📐️estimates 🚪️gate 🪜️phases 🤝️handover; do nice -n 10 bun test --timeout 120000 "./🧪️tests/$f/🟦️.ts"; done; nice -n 10 bun test --timeout 120000 ./🧪️tests/🟦️.ts) >"$out" 2>&1
(cd "$LIB" && for f in 🌤️followup 🔼️upgrade 🧮️arguments 🏊️farm 📈️memory-fit 🔁️restart 🪟️partial 🪜️phases; do nice -n 10 bun test --timeout 120000 "./🌡️host-capacity/🧪️tests/$f/🟦️.ts"; done; nice -n 10 bun test --timeout 120000 ./🌡️host-capacity/🧪️tests/🟦️.ts) >>"$out" 2>&1
grep -E '^ *[0-9]+ (pass|fail)$' "$out" | awk '{s[$2]+=$1} END {print "pass", s["pass"], "fail", s["fail"]}'
unexpected=$(grep '^(fail)' "$out" | grep -v -e 'a grant is lent only while the process is inside declared queue waits' -e 'not the child of the offering process does not take the grant over' -e 'a real replacement process takes the grant' -e 'the restart wrapper hands the grant of a stale command')
rm -f "$out"
if [ -n "$unexpected" ]; then echo "$unexpected"; exit 1; fi
echo "quietfc-mutual VERIFIED"
