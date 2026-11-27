#!/bin/zsh
# 🧪️ Runs `trial.ts` over the given assets inside the grant of its caller: trial.sh <ids…>
source "${0:A:h}/env.sh"
zsh "$W/run.sh" trial.ts "$@" 2>&1 | cut -c1-400
