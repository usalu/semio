#!/bin/zsh
# 🏋️ The two heavy proof steps of assets under ONE grant of the memory queue instead of one admission per step (an admission costs 15 to 40 minutes on the shared host): the check CLI of each id, then the matrix of each (the steps `check` and `matrix` of proof.sh, one after the other, summary lines in $GEN/proof/summary.log): big.sh <id>…   (BIG_MB is the reservation, 1200 MB by the queue rule of 2026-11-25 09:20; NOMATRIX=1 or NOCHECK=1 leaves a step out; inside the grant the script calls itself with INSIDE=1)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" ${BIG_MB:-1000} ${BIG_TIMEOUT:-21600} "$W" "${0:A}" "$@"; fi
[[ -n "$NOCHECK" ]] || for a in "$@"; do INSIDE=check "$W/proof.sh" "$a"; done
[[ -n "$NOMATRIX" ]] || for a in "$@"; do INSIDE=matrix "$W/proof.sh" "$a"; done
