#!/bin/zsh
# 🚦️ Runs the fixer's gate (MR/gate.ts --twins, every pose, clip instant and transition) for the assets given (default: the whole batch MH-2), one after the other, each log in $GEN/<dir>/<id>.log: DIR=gate0 gate-all.sh [<id>…]
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
d="$GEN/${DIR:-gate}"; mkdir -p "$d"
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($(cut -f2 "$U/🧰️matrix/batches/MH-2.tsv"))
for a in $ids; do
  s=$SECONDS
  ("$W/q.sh" 2500 1800 "$T" "$B" "$M/gate.ts" "${SRC:+$GEN/$SRC/}$a${SRC:+.json}" --twins ${=GATE_ARGS}) > "$d/$a.log" 2>&1
  echo "$a exit $? $((SECONDS-s))s $(tail -1 "$d/$a.log")"
done
