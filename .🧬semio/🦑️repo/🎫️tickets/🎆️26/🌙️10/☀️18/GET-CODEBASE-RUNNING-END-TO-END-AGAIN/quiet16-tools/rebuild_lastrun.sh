#!/bin/sh
# 🔁 quiet16: rebuilds batch `lastrun` on a mirror that holds the current live tree with agents16 and `hold` applied: the anchored edits (edit_lastrun.py), the regenerated oracle fixtures (estimates with numpy, upgrade, follow-up, the library quiet replay) and the flat batch (make.py). Run it through `mirror.py run <name> -- sh rebuild_lastrun.sh <mirror root>` so the oracles cannot write into the live tree; the batch itself is written by make.py afterwards from outside: python3 -I make.py lastrun <mirror root>
set -e
root=${1:?root}; here=$(cd "$(dirname "$0")" && pwd)
python3 -I "$here/edit_lastrun.py" "$root"
H="$root/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests"; L="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests"
(cd "$H/📐️estimates" && python3 🐍️oracle/🐍️.py)
for oracle in "$H/🔼️upgrade" "$H/🌤️followup" "$L/🤫️quiet"; do (cd "$oracle" && python3 -I 🐍️oracle/🐍️.py); done
for oracle in "$H/🪜️phases" "$H/🔼️upgrade/🌱️grow" "$H/🧾️blockers"; do (cd "$oracle" && python3 -I 🐍️oracle/🐍️.py --check); done
