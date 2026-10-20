#!/bin/zsh
# usage: live.sh apply|real|env|all|vitest   (run only after the coordinator's GO; <= 2 concurrent bun processes)
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/testfix-defect-tools"
case "$1" in
  apply) python3 $T/apply.py --write ;;
  real) python3 $T/run.py $T/suites-real21.txt $T/../🗑️generated/tfd/real.jsonl --jobs 2 ;;
  env) python3 $T/run.py $T/suites-env168.txt $T/../🗑️generated/tfd/env.jsonl --jobs 2 ;;
  all) python3 $T/run.py $T/suites-all.txt $T/../🗑️generated/tfd/all.jsonl --jobs 2 ;;
esac
