#!/bin/sh
# 🧾 Laws of batch `behind` (after `allrun` and `behind` are applied): sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests
(cd "$H/🧾️blockers" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$H/🔼️upgrade" && python3 -I 🐍️oracle/🐍️.py --check)
bun test --timeout 120000 "$H/🧾️blockers/🟦️.ts" "$H/🟦️.ts" "$H/🚪️gate/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🔼️upgrade/🚪️gate/🟦️.ts" "$H/🔼️upgrade/👥️agents/🟦️.ts" "$H/🌤️followup/🟦️.ts" "$L/🧾️label/🟦️.ts" "$L/👥️agents/🟦️.ts" "$L/🌤️followup/🟦️.ts" "$L/🔼️upgrade/🟦️.ts" "$L/🪟️partial/🟦️.ts"
echo "VERIFY OK"
