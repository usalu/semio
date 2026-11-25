#!/bin/zsh
# ⏳️ Waits until the worker q.sh has finished the job of the given number and prints the last lines of a proof: wait.sh <n> [<id>]
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
until grep -q "^done $1 " "$GEN/q.log"; do sleep 15; done; date "+%H:%M"; [[ -n "$2" ]] && { grep -c "exit 0" "$GEN/proof/$2.out"; grep -v "exit 0\|OK$\| 0 issues\| 0 findings\|check: ✓" "$GEN/proof/$2.out" | cut -c1-150; }
