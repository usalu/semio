#!/bin/sh
# 👥 Laws of batch `allrun` after apply (the estimates oracle needs numpy, cross-checked third party; it rewrites its fixture, which stage.py check then proves byte-equal to the batch): sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK). Compare failure sets with the same run on the base (C/agents16-tools/suites.sh) for the envelope tests that fail on base too.
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests
(cd "$H/🔼️upgrade" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$H/🔼️upgrade/🌱️grow" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$H/🌤️followup" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$H/📐️estimates" && python3 🐍️oracle/🐍️.py)
python3 -I "$here/stage.py" check --root "$root"
bun test --timeout 120000 "$H/🔼️upgrade/👥️agents/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🔼️upgrade/🤝️mutual/🟦️.ts" "$H/🔼️upgrade/🌱️grow/🟦️.ts" "$H/🔼️upgrade/🚪️gate/🟦️.ts" "$H/🌤️followup/🟦️.ts" "$H/📐️estimates/🟦️.ts" "$H/🪜️phases/🟦️.ts" "$H/🚪️gate/🟦️.ts" "$H/🔏️fence/🟦️.ts" "$H/🟦️.ts" "$L/👥️agents/🟦️.ts" "$L/🌤️followup/🟦️.ts" "$L/🌱️grow/🟦️.ts" "$L/🔼️upgrade/🟦️.ts" "$L/🪟️partial/🟦️.ts" "$L/🏊️farm/🟦️.ts" "$L/📈️memory-fit/🟦️.ts" "$L/🟦️.ts"
echo "VERIFY OK"
