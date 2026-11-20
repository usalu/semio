#!/bin/sh
# 🚚 Builds the round-3 candidate of humans/worker-carrying-box and, only when it validates, writes the root fixes into the catalog asset, splices the volumes and validates the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/worker-carrying-box/🔣️.json"
TAIL=1 ./r3-worker-carrying-box-build.sh | grep -q "^VALID" || { echo "candidate invalid: nothing written"; exit 1; }
python3 r3-worker-carrying-box-roots.py --apply | tail -1
python3 volumes-insert.py volumes/humans/worker-carrying-box.json "$C" | tail -1
$BUN r3-validate.ts humans/worker-carrying-box --catalog-only | tail -3
