#!/bin/zsh
# 💤️ Waits up to <seconds> until the job log or the matrix summary of batch MH-2 grows or the given job has output: wait.sh <seconds> [job]
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; n=$(cat $GEN/jobs/log.txt $GEN/proof/matrix-summary.txt | wc -l); e=$((SECONDS+$1))
while (( SECONDS < e )); do sleep 5; (( $(cat $GEN/jobs/log.txt $GEN/proof/matrix-summary.txt | wc -l) > n )) && break; done
date +%H:%M; tail -2 $GEN/jobs/log.txt; tail -2 $GEN/proof/matrix-summary.txt | cut -c1-250; ls $GEN/jobs/*.job | xargs -n1 basename
