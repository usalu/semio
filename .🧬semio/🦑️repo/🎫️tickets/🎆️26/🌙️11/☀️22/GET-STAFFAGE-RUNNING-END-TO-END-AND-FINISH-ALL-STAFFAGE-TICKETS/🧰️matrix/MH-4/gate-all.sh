#!/bin/zsh
# 🚦️ Runs the MR gate (every pose, clip instant and transition, with twins and the reported tier) one asset after the other: gate-all.sh <out-folder-name> [<id|file>…]   (default: the catalog files of the batch; logs in $GEN/<out-folder-name>)
source "${0:A:h}/env.sh"
source "$W/lock.sh"
out="$GEN/$1"; shift; mkdir -p "$out"
list=("$@"); (( $# == 0 )) && list=($IDS)
for a in $list; do
  name="${${a:t}%.json}"
  (cd "$T" && timeout 1800 "$B" "$W/gate.ts" "$a" --twins --soft ${=GATE_ARGS}) > "$out/$name.log" 2>&1
  echo "$name exit $? $(tail -1 "$out/$name.log")"
done
