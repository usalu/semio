#!/bin/zsh
# ⏳️ Waits up to ${LIMIT_S:-540} s until every job named is done in a lane of this fixer, then prints the tail of the outputs: until.sh <job part>…
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"; setopt null_glob
end=$(( $(date +%s) + ${LIMIT_S:-540} ))
for j in "$@"; do
  while true; do d=("$GEN"/jobs*/*$j*.done); [[ ${#d} -gt 0 || $(date +%s) -gt $end ]] && break; sleep 3; done
done
date; tail -qn2 "$GEN"/jobs*/worker.log; for j in "$@"; do for o in "$GEN"/jobs*/*$j*.out; do echo "--- ${o:t}"; tail -${LINES_N:-60} "$o" | cut -c1-260; done; done
