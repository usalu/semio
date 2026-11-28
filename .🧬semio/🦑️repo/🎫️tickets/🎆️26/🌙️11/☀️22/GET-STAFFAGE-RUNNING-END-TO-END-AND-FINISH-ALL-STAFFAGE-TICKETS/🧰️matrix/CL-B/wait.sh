#!/bin/zsh
# ⏳️ Waits up to 170 s for the named queue runs of lane CL-B to be recorded in `$GEN/log/runs.txt` and prints their lines: wait.sh <name>…
source "${0:A:h}/env.sh"
for i in {1..17}; do ok=1; for n in "$@"; do grep -q " $n exit " "$GEN/log/runs.txt" || ok=0; done; [[ $ok -eq 1 ]] && break; sleep 10; done
for n in "$@"; do grep " $n exit " "$GEN/log/runs.txt"; done
