#!/bin/zsh
# 🤖️ The greedy silhouette repair over people, the finest scale first; continues the ops of 🗑️generated/REP-B/ops. autoall.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do MAXSEC=${MAXSEC:-150} "$W/auto.sh" "$a"; done
echo AUTO-DONE
