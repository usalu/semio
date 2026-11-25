#!/bin/zsh
# ⛓️ A list of tool steps inside ONE grant (start it with qsh), strictly one after the other: every line of the list file is one zsh command with the paths of ../MR/env.sh set (B the pinned bun, S, T, U, M, GEN) and `here` this folder; the in-*.sh scripts of this folder may be steps. Each step prints `== <line>` before and its exit code after: QMEM=<the need of the heaviest step> QTIME=<s> qsh in-chain.sh <list.txt>
[[ -n "$MH3_GRANT" ]] || { echo "in-chain.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
export here="${0:A:h}"
source "$here/../MR/env.sh"
while IFS= read -r line <&3; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  echo "== $line"
  eval "$line" < /dev/null
  echo "-- exit $?"
done 3< "$1"
echo "chain finished"
