#!/bin/zsh
# 🛬️ Writes the repair of an asset into the catalog and proves it: fit.ts with ops/<id>.json and --write (refused when the file changed on disk), reruns until a run changes nothing (at most four), then the whole proof of `../MR/prove.sh` with the matrix and the cell summary: land.sh <id>…   (NOMATRIX=1 leaves the matrix out)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  echo "=== $a $(date +%H:%M)"
  for i in 1 2 3 4; do
    last="$(cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --write | tail -1)"; echo "write $i: $last"
    [[ "$last" == *" 0 changed something"* ]] && break
  done
  if [[ -z "$NOMATRIX" ]]; then MATRIX=1 "$M/prove.sh" "$a"; python3 "$M/cells.py" "$GEN/matrix-$a" | cut -c1-300; else "$M/prove.sh" "$a"; fi
  echo "=== $a done $(date +%H:%M)"
done
