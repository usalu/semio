#!/bin/zsh
# ⏳️ Waits up to 170 s for a change in the two lanes of this fixer, then prints one short state: await.sh returns early when a further job of the worker is done or a further matrix log of mx.sh gained its `guarded:` line.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"; setopt null_glob
count() { local d=("$GEN"/jobs*/*.done) m=0 q; for q in "$GEN"/proof/*-matrix.queue.log; do grep -q '^guarded:' "${q%.queue.log}.log" 2>/dev/null && m=$((m + 1)); done; echo "${#d}/$m"; }
start="$(count)"; i=0
while [[ "$(count)" == "$start" && $i -lt 34 ]]; do sleep 5; i=$((i + 1)); done
r=("$GEN"/jobs*/*.running); j=("$GEN"/jobs*/*.job)
echo "$(date +%H:%M) jobs/matrices done $(count) (was $start); worker: $(tail -q -n1 "$GEN"/jobs*/worker.log | cut -c9-40 | tr "\n" ";"); running: ${r:t}; waiting jobs ${#j}"
for q in "$GEN"/proof/*-matrix.queue.log; do log="${q%.queue.log}.log"; grep -q '^guarded:' "$log" 2>/dev/null || echo "matrix in flight: ${log:t} $(tail -1 "$log" 2>/dev/null | cut -c1-110)"; done
