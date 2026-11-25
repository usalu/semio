#!/bin/zsh
# 🔦️ The search of auto.py for several people, each under ONE grant of 2500 MB (q.sh with MH5_GRANT=1; the fits, model gates and guards of the search run one after the other inside it): search.sh [--fresh] <id>… prints the KEPT, OPEN and done lines of each log.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
fresh=(); [[ "$1" == --fresh ]] && { fresh=(--fresh); shift; }
cd "$W"
for a in "$@"; do
  MEM=2500 "$W/q.sh" env MH5_GRANT=1 python3 "$W/auto.py" "$a" $fresh > /dev/null 2>&1
  echo "== $a $(date +%H:%M)"; grep -E "^(KEPT|OPEN)| done: " "$GEN/auto/$a.log" | cut -c1-${COLS:-200}
done
