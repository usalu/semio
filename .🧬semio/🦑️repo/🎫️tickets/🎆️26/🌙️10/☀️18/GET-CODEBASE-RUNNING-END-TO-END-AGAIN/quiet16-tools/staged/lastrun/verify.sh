#!/bin/sh
# 🔁 Laws of batch `lastrun` (after `hold`) applied: sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK). Every Python oracle whose fixture the forecast rule or the policy reaches is checked (the estimates oracle with numpy rewrites its fixture, which stage.py check then proves byte-equal to the batch), the batch must read `applied`, the forecast, follow-up, upgrade, estimates, phases and quiet laws run, and the graph-free simulation must show every hot no-op forecast quiet and waiting 0 s for memory on every host. The framework root suite (`🧪️tests/🟦️.ts`, decision vectors over the stripped policy) runs in C/quiet16-tools/suites.sh: its SQLite-oracle cases time out at load 70+ on base and batch alike.
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; tools=$(cd "$here/../.." && pwd)
cd "$root"
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests
(cd "$H/📐️estimates" && python3 🐍️oracle/🐍️.py)
for oracle in "$H/🔼️upgrade" "$H/🔼️upgrade/🌱️grow" "$H/🌤️followup" "$H/🪜️phases" "$H/🧾️blockers" "$L/🤫️quiet"; do (cd "$oracle" && python3 -I 🐍️oracle/🐍️.py --check); done
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "VERIFY FAIL: batch not applied"; exit 1; }
python3 -I "$tools/staged/hold/stage.py" check --root "$root" | grep -v '^applied' | grep -v '🤫️quiet/🧫️fixtures' && { echo "VERIFY FAIL: hold not applied"; exit 1; }
bun test "$H/🌤️followup/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/📐️estimates/🟦️.ts" "$H/🪜️phases/🟦️.ts" "$H/🧾️blockers/🟦️.ts" "$H/🤫️quiet/🟦️.ts" "$L/🌤️followup/🟦️.ts" "$L/🔼️upgrade/🟦️.ts" "$L/🤫️quiet/🟦️.ts" "$L/🪜️phases/🟦️.ts" "$L/📈️memory-fit/🟦️.ts" "$L/🪟️partial/🟦️.ts" "$L/👥️agents/🟦️.ts" "$L/🟦️.ts"
out=$(mktemp -d); QUIET16_OUT=$out python3 -I "$tools/sim/table.py" "$root" verify --law --quiet --lastrun --busy > "$out/table.md" || { cat "$out/table.md"; rm -rf "$out"; echo "VERIFY FAIL: simulation"; exit 1; }
tail -1 "$out/table.md"; rm -rf "$out"
echo "VERIFY OK"
