#!/bin/sh
# 🔬 Laws of batch `narrow` after apply: sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK). Compare failure sets with the same run on the base: the host-capacity suites (the 'grow' g31 replay is load-flaky on base too).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}
cd "$root"
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
(cd "$H/🔼️upgrade" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$H/🔼️upgrade/🌱️grow" && python3 -I 🐍️oracle/🐍️.py --check)
python3 -I "$here/stage.py" check --root "$root"
bun test --timeout 120000 "$H/🔼️upgrade/👥️agents/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🔼️upgrade/🤝️mutual/🟦️.ts" "$H/🔼️upgrade/🌱️grow/🟦️.ts" "$H/🔼️upgrade/🚪️gate/🟦️.ts" "$H/🌤️followup/🟦️.ts" "$H/🧾️blockers/🟦️.ts" "$H/🟦️.ts"
echo "VERIFY OK"
