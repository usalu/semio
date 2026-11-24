#!/bin/zsh
# 🖊️ Writes the repair of an asset into the catalog, the last step of its chain: runs `fit2.ts` with `ops/<id>.json` on the catalog file until a run changes nothing (at most five runs; `fit2.ts` refuses to write when the file changed on disk while it ran). Idempotent: on a repaired file the first run reports `0 changed something`. apply.sh <id>
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; mkdir -p "$GEN/apply"
for i in 1 2 3 4 5; do
  (cd "$T" && timeout 600 "$B" "$W/fit2.ts" "$a" --ops "$W/ops/$a.json" --write) > "$GEN/apply/$a.$i.log" 2>&1
  echo "run $i: $(tail -2 "$GEN/apply/$a.$i.log" | tr '\n' ' ' | cut -c1-300)"
  grep -q " 0 changed something" "$GEN/apply/$a.$i.log" && break
done
