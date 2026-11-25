#!/bin/zsh
# ⏳️ Waits until a job log of the worker is complete (or the seconds are over) and prints it: await.sh <job name> [<seconds>]
source "${0:h}/env.sh"
f="$GEN/jobs/done/$1.log"; n=0
until [[ -f "$f" || $n -ge ${2:-110} ]]; do sleep 2; n=$((n + 2)); done
[[ -f "$f" ]] && grep -v '^spec ' "$f" | cut -c1-260 || { echo "not yet: $(ls "$GEN/jobs/done" | tail -2 | tr '\n' ' ') filler: $(tail -1 "$GEN/filler.log" | cut -c1-80)"; }
