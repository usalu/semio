#!/bin/zsh
# 🧮️ The budget facts of the whole catalog (`budgets.ts`, triangles of every scene, no file audit), resumable, inside the grant of its caller: scan.sh [--glb] writes `$GEN/budgets.jsonl` (or `budgets-glb.jsonl`).
source "${0:A:h}/env.sh"
zsh "$W/run.sh" budgets.ts "$GEN/budgets${1:+-glb}.jsonl" "$@"
