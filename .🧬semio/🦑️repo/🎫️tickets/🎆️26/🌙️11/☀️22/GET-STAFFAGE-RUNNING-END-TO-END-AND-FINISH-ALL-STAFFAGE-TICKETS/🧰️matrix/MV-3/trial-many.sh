#!/bin/zsh
# 🧪️ The search run (`trial.sh`) of several plants of batch MV-3, one after the other; meant to run under one grant (`grant.sh 1500 <timeout> trial-many.sh <id>…`). One block of result lines per plant is appended to `$GEN/log/trials.txt`.
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do { echo "== $a $(date +%H:%M)"; "$W/trial.sh" "$a"; } >> "$GEN/log/trials.txt" 2>&1; done
echo "trials done: $*"
