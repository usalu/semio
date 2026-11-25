#!/bin/zsh
# 💤️ One terse wait of batch MH-2: blocks up to 170 s for a new line in the job log or the matrix summary and prints one line: the time, the finished gate proofs of the running job, failed exits, the last job and matrix lines.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; setopt null_glob
"$W/st.sh" ${1:-168} > /dev/null 2>&1
cur="$(ls -t "$GEN"/jobs/*.out | grep -v runner | head -1)"
print -r -- "$(date +%H:%M) ${cur:t}: $(grep -cE 'scan exit|tests .* exit|TRIALDONE|=== B' "$cur") steps, bad: $(grep -E 'exit [1-9]' "$cur" | cut -c1-120 | tr '\n' ';') | last job: $(tail -1 "$GEN/jobs/log.txt" | cut -c1-60) | matrix: $(grep '=== C' "$GEN/proof/matrix-summary.txt" 2>/dev/null | tail -1 | cut -c1-150)"
