#!/bin/sh
# 🤫 Laws of batch `hold` after apply: sh verify.sh <repository or mirror root>  (exit 0 = VERIFY OK). The library oracle is checked (python3, stdlib + the framework estimates oracle), the batch must read `applied`, the new and changed laws and every host-capacity suite they touch run, the Bazel action-width gate equals its Python oracle, and the graph-free agents16-derived simulation must show every hot no-op waiting 0 s for memory on all hosts, the declared 16 GB laptop included, and (audit9 §hold, `busy2`/`busy3`) quiet-forecast runs that turn busy at once never take the host more than one job below its floor. Compare the failure set with C/quiet16-tools/suites.sh on the base (real-process lending/handover tests fail under the mirror seal on base too; the library g31 grow replay is timing-flaky under load on base and batch alike and is left to suites.sh).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; tools=$(cd "$here/../.." && pwd)
cd "$root"
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
G=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests
B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests
(cd "$L/🤫️quiet" && python3 -I 🐍️oracle/🐍️.py --check)
(cd "$B/🚦️width" && python3 -I 🐍️oracle/🐍️.py --check)
python3 -I "$here/stage.py" check --root "$root" | grep -v '^applied' && { echo "VERIFY FAIL: batch not applied"; exit 1; }
bun test "$B/🚦️width/🟦️.ts" "$B/🧷️births/🟦️.ts" "$B/🟦️.ts" "$H/🤫️quiet/🟦️.ts" "$L/🤫️quiet/🟦️.ts" "$H/🔼️upgrade/🚪️gate/🟦️.ts" "$H/🔼️upgrade/👥️agents/🟦️.ts" "$G/🔼️upgrade-before-custody/🟦️.ts" "$G/🎟️prefetch-booking/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🔼️upgrade/🤝️mutual/🟦️.ts" "$H/🔼️upgrade/🌱️grow/🟦️.ts" "$H/🌤️followup/🟦️.ts" "$L/🔼️upgrade/🟦️.ts" "$L/🌤️followup/🟦️.ts" "$L/🪜️phases/🟦️.ts" "$L/👥️agents/🟦️.ts"
out=$(mktemp -d); QUIET16_OUT=$out python3 -I "$tools/sim/table.py" "$root" verify --law --quiet --busy > "$out/table.md" || { cat "$out/table.md"; rm -rf "$out"; echo "VERIFY FAIL: simulation"; exit 1; }
tail -1 "$out/table.md"; rm -rf "$out"
echo "VERIFY OK"
