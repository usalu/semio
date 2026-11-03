#!/bin/bash
# 🚦 Pre-build static gate (compile review H). Usage: gate.sh [en1994|en1990|en1991|shared|all] ; no cargo, no rustc.
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
G="$T/🧰️scripts/compile-gate-h"
O="$T/🗑️generated/compile-review-h/gate"
mkdir -p "$O"
cd /home/deck/semio
PY=".venv/bin/python -I"
N="✏️s/🔌️plugins/📕️norm"
case "${1:-all}" in
  en1994) roots=("$N/🗿️artifacts/🧩️en1994");;
  en1990) roots=("$N/🗿️artifacts/⚖️en1990");;
  en1991) roots=("$N/🗿️artifacts/🏋️en1991");;
  shared) roots=("$N/📇️registry" "$N/⚖️compliance" "$N/🎬️structural-scene" "$N/🪟️results" "$N/🪟️viewport" "$N/🧮️mechanics");;
  *) roots=("$N/🗿️artifacts/🧩️en1994" "$N/🗿️artifacts/⚖️en1990" "$N/🗿️artifacts/🏋️en1991" "$N/📇️registry" "$N/⚖️compliance" "$N/🎬️structural-scene" "$N/🪟️results" "$N/🪟️viewport" "$N/🧮️mechanics");;
esac
echo "== cargo features / workspace deps (featck)"; $PY "$G/featck.py" "${roots[@]}"
echo "== import vs local definition clashes (collide)"; $PY "$G/collide.py" "${roots[@]}" | grep -v "use diagrams::diagrams\|use handles::handles\|members::handles"
echo "== row_patch! sets vs model structs (patchck)"; for r in "${roots[@]}"; do $PY "$G/patchck.py" "$r"; done
echo "== Enum::Variant that does not exist (enumck)"; $PY "$G/enumck.py" "${roots[@]}" | grep -v "variants: \[\])"
echo "== methods / fields that exist nowhere (methck, fieldck)"; $PY "$G/methck.py" "$O" "${roots[@]}" 2>/dev/null
