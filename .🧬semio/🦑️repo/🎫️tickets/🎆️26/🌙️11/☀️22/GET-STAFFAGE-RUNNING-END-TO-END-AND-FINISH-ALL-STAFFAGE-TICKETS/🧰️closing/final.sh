#!/usr/bin/env bash
# 🏁️ Final model-catalog pass after closing round 2, resumable: final.sh <category,...|all> [assets per shard] — one capacity request at a time, JSON and logs in <U>/🗑️generated/closing/final/
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
U="$(cd "$HERE/.." && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
S="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1)"
F="$U/🗑️generated/closing/final"
mkdir -p "$F/shards"
python3 - "$S" "$F/shards" "$1" "${2:-12}" <<'PY'
import os, sys
root, out, want, size = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
cat = root + "/🗂️catalog"
for c in sorted(os.listdir(cat)):
    if c in ("🌤️environments", "🖨️print-profiles") or not os.path.isdir(f"{cat}/{c}"): continue
    name = c.split("️")[-1]
    if want != "all" and name not in want.split(","): continue
    ids = sorted(d for d in os.listdir(f"{cat}/{c}") if os.path.isdir(f"{cat}/{c}/{d}"))
    for i in range(0, len(ids), size):
        p = f"{out}/{name}-{i // size:02d}"
        if not os.path.exists(p): open(p, "w").write(",".join(ids[i:i + size]))
PY
for shard in "$F"/shards/*; do
  label="$(basename "$shard")"
  [ -f "$F/$label.json" ] && continue
  started=$(date +%s)
  (cd "$ROOT" && SEMIO_STAFFAGE_MODEL_ASSETS="$(cat "$shard")" repo capacity run --memory=3600 -- timeout 5400 "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --reporter=default --reporter=json --outputFile.json="$F/$label.json" --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000) > "$F/$label.log" 2>&1
  echo "$label exit $? seconds $(( $(date +%s) - started ))" | tee -a "$F/summary.txt"
done
