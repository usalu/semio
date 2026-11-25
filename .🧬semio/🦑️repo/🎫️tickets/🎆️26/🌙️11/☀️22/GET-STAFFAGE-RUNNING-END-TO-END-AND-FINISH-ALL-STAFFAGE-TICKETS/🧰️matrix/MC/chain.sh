#!/bin/zsh
# ⛓️ Runs steps of batch MC one after the other inside ONE queue grant (the queue admits a fixer rarely, so a grant is used for a whole chain; still one tool process at a time): chain.sh <step> [<step>…], each step one quoted command line of a script of this folder, for example chain.sh "try.sh city-car" "suites.sh before".   (run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
for step in "$@"; do
  echo "[DEBUG] chain step: $step ($(date +%H:%M:%S))"
  eval "\"$W\"/$step"
done
echo "[DEBUG] chain done ($(date +%H:%M:%S))"
