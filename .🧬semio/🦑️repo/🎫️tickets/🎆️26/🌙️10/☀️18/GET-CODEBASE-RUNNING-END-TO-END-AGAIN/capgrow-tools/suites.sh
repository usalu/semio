#!/bin/sh
# 🧪️ capgrow: runs every suite the batch touches, one file at a time, and prints one line per file: <pass> <fail> <file>.
R=${1:?root}
HC="$R/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
CU="$R/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity"
LIB="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
CARGO="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo"
run(){ out=$(cd "$1" && nice -n 10 bun test --timeout 120000 "$2" 2>&1); p=$(printf '%s\n' "$out" | sed -n 's/^ *\([0-9][0-9]*\) pass$/\1/p' | tail -1); f=$(printf '%s\n' "$out" | sed -n 's/^ *\([0-9][0-9]*\) fail$/\1/p' | tail -1); echo "${p:-0} ${f:-0} $2"; printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/^/   /'; }
for o in 🔼️upgrade 🌤️followup 🪜️phases; do (cd "$HC/🧪️tests/$o/🐍️oracle" && echo "oracle $o: $(python3 ./🐍️.py --check)"); done
[ -f "$HC/🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle/🐍️.py" ] && (cd "$HC/🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle" && echo "oracle grow: $(python3 ./🐍️.py --check)")
for f in 🌤️followup 🔼️upgrade 🔼️upgrade/🤝️mutual 🔼️upgrade/🚪️gate 🔼️upgrade/🌱️grow 📐️estimates 📡️sampling 🚪️gate 🪜️phases 🤝️handover 🧭️liveness 🔮️sqlite 🪟️windows; do [ -f "$HC/🧪️tests/$f/🟦️.ts" ] && run "$HC" "./🧪️tests/$f/🟦️.ts"; done
run "$HC" ./🧪️tests/🟦️.ts
run "$CU" ./🧮️units/🧪️tests/🟦️.ts
for f in 🌤️followup 🔼️upgrade 🧮️arguments 🏊️farm 📈️memory-fit 🔁️restart 🪟️partial 🪜️phases 🌱️grow; do [ -f "$LIB/🌡️host-capacity/🧪️tests/$f/🟦️.ts" ] && run "$LIB" "./🌡️host-capacity/🧪️tests/$f/🟦️.ts"; done
run "$LIB" ./🌡️host-capacity/🧪️tests/🟦️.ts
run "$CARGO" ./🚪️launcher/🧪️tests/🟦️.ts
run "$CARGO" ./🧪️testing/📦️executable/🧪️tests/🌿️runtime-selection/🟦️.ts
run "$R/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo" ./🎯️exact/🧪️tests/📥️inputs/🟦️.ts
