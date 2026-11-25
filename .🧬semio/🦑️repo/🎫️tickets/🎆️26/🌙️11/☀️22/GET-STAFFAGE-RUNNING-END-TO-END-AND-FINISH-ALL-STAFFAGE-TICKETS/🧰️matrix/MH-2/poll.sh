#!/bin/zsh
# ⏳️ Waits up to 165 s until the job log or the matrix summary of batch MH-2 grows, then prints the new lines: poll.sh
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; a=$(wc -l < $GEN/jobs/log.txt); b=$(grep -c '=== C' $GEN/proof/matrix-summary.txt); e=$((SECONDS+165))
while (( SECONDS < e )); do (( $(wc -l < $GEN/jobs/log.txt) > a || $(grep -c '=== C' $GEN/proof/matrix-summary.txt) > b )) && break; sleep 5; done
date +%H:%M; tail -n +$((a+1)) $GEN/jobs/log.txt | cut -c1-160; grep '=== C' $GEN/proof/matrix-summary.txt | tail -n +$((b+1)) | cut -c1-220
