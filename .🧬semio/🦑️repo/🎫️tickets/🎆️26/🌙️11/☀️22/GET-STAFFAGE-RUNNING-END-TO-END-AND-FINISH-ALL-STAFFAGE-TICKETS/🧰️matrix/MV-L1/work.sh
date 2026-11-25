#!/bin/zsh
# 🪛️ One reservation for a chain of short trial steps of batch MV-L1, one process at a time: every `<name>.job` (a zsh script that uses the variables of `env.sh` and `q.sh` with INSIDE=1) dropped into `$GEN/jobs` is run in the order of its name, its output goes to `<name>.out`, and it is renamed to `.done`. The reservation ends LINGER seconds (default 240) after the last job, or at once when `$GEN/jobs/stop` exists. work.sh [megabytes]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/jobs" "$GEN/try" "$GEN/log" "$GEN/sheets" "$W/ops"; export MEMLOG="$GEN/log/mem-work.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="${1:-900}" -- zsh -c '
idle=0
while [[ $idle -lt ${LINGER:-240} && ! -e "$GEN/jobs/stop" ]]; do
  next=("$GEN"/jobs/*.job(N))
  if (( $#next )); then idle=0; j="$next[1]"; mv "$j" "${j:r}.run"; zsh "${j:r}.run" > "${j:r}.out" 2>&1; echo "exit $?" >> "${j:r}.out"; mv "${j:r}.run" "${j:r}.done"; else sleep 2; idle=$((idle+2)); fi
done
rm -f "$GEN/jobs/stop"' work
