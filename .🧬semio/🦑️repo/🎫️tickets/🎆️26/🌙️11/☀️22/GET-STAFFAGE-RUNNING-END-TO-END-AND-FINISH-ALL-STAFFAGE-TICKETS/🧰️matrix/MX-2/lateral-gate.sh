#!/bin/zsh
# 🚦️ The fixer's gate of the assets that author `lateralScale` (trial copies of lateral.py), each on the catalog document (the member read) and on the copy without it (what the engine drew before D54): lateral-gate.sh [<id>…] appends to 🗑️generated/matrix/MX-2/lateral-gate.txt; runs inside a grant that is already held (a lane job).
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"
ids=("$@"); [[ $# -eq 0 ]] && ids=($(python3 "$W/lateral.py" "$GEN/lateral"))
for a in $ids; do
  for mode in read ignored; do
    doc="$a"; [[ $mode == ignored ]] && doc="$GEN/lateral/$a.json"
    (cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$doc") > "$GEN/lateral/gate-$a-$mode.txt" 2>&1
    echo "$a $mode: $(tail -1 "$GEN/lateral/gate-$a-$mode.txt" | cut -c1-200)" | tee -a "$GEN/lateral-gate.txt"
  done
done
