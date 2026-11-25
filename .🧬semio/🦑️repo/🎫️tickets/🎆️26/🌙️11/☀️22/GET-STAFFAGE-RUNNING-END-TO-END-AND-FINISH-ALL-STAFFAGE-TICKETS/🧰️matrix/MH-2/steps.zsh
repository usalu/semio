#!/bin/zsh
# 🪜️ Steps of one grant: runs the scripts $GEN/steps/*.zsh in name order, output to the same name with .out, script moved to $GEN/steps/done; ends after <seconds> without a new step. steps.zsh <seconds>
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; setopt nullglob; p="$GEN/steps"; mkdir -p "$p/done"; e=$((SECONDS+$1))
while (( SECONDS < e )); do f=("$p"/*.zsh); if (( $#f )); then /bin/zsh "$f[1]" > "${f[1]:r}.out" 2>&1; echo "STEPDONE $?" >> "${f[1]:r}.out"; mv "$f[1]" "$p/done/"; e=$((SECONDS+$1)); else sleep 2; fi; done
