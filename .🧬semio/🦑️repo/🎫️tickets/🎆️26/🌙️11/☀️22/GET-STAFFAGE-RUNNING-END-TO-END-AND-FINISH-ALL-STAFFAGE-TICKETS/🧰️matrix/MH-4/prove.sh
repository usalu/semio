#!/bin/zsh
# 🩺️ The proof of MR (`../MR/prove.sh`) for assets of this batch, one after the other under the lock of the batch: prove.sh <id>…   (MATRIX=1 adds the matrix of each asset; the lines go to $GEN/proof/<id>.out too)
source "${0:A:h}/env.sh"
source "$W/lock.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  s=$(date +%s)
  BATCH=MH-4 "$M/prove.sh" "$a" 2>&1 | tee "$GEN/proof/$a.out"
  [[ -n "$MATRIX" ]] && python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300 | tee -a "$GEN/proof/$a.out"
  echo "$a proof took $(( $(date +%s) - s )) s" | tee -a "$GEN/proof/$a.out"
done
