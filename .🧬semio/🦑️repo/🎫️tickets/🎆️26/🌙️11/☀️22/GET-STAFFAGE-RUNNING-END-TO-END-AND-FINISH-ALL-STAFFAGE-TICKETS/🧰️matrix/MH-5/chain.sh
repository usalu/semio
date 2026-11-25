#!/bin/zsh
# ⛓️ One step of land.sh for several people, each under ONE grant of the machine-wide queue (q.sh with MH5_GRANT=1: the processes of the step run one after the other inside the one reservation instead of queueing each for itself): chain.sh <check|write|prove|matrix|all> <id>… ; every result goes to $GEN/land/<id>.<step>.out and its last lines to stdout. The matrix reserves 7000 MB, every other step 5000 MB.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
step="$1"; shift; mkdir -p "$GEN/land"
for a in "$@"; do
  mem=5000; [[ "$step" == matrix ]] && mem=7000
  MEM=$mem "$W/q.sh" env MH5_GRANT=1 ALLOW_HARD="$ALLOW_HARD" "$W/land.sh" "$step" "$a" > "$GEN/land/$a.$step.out" 2>&1
  echo "== $a $step $(date +%H:%M)"; grep -v '^\[capacity\]' "$GEN/land/$a.$step.out" | tail -${TAIL:-16} | cut -c1-240
done
