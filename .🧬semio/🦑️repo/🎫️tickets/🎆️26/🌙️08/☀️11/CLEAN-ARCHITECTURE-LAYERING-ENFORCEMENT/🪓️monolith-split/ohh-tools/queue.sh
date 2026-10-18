#!/bin/zsh
# usage: queue.sh <wait-for-result-json> "<family> <cfgs> <label>" ...   runs pos.py sequentially (niced) after the awaited result exists
cd "$(dirname "$0")/pos" || exit 1
ulimit -n 65536
export POS_PREFIX=ohh
GEN="$(dirname "$0")/../../🗑️generated"
GEN="$(cd .. && pwd)/../🗑️generated"
wait_for="$1"; shift
while [ ! -f "$GEN/$wait_for" ]; do sleep 20; done
for job in "$@"; do
  set -- ${=job}
  nice -n 5 python3 pos.py run ../families/$1.json --cfgs $2 --label $3 --no-base --iters 2 > "$GEN/ohh-$1-$3-console.log" 2>&1
done
