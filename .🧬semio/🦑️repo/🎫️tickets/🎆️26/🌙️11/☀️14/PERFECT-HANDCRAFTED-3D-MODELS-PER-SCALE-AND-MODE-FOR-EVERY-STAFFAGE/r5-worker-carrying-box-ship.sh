#!/bin/sh
# 🚢 Round 5 of humans/worker-carrying-box: root draft, volumes, 2D redraw from the volumes, root draft and volumes again, validation of the candidate; with --apply also writes the catalog (only when the candidate is VALID), splices the volumes and runs the product volume check and the full validation.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/worker-carrying-box/🔣️.json"
O="🗑️generated/r5-worker-carrying-box"; mkdir -p "$O"
python3 r5-worker-carrying-box-roots.py > /dev/null
python3 r5-worker-carrying-box-model.py
$BUN r5-worker-carrying-box-draw2d.ts | tail -1
python3 r5-worker-carrying-box-roots.py > /dev/null
python3 r5-worker-carrying-box-model.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-worker-carrying-box/🔣️.json" > "$O/candidate.txt" || true
grep -v "^VALID\|^INVALID" "$O/candidate.txt" | cut -c1-220 || true
tail -1 "$O/candidate.txt"
[ "$1" = "--apply" ] || exit 0
grep -q "^VALID" "$O/candidate.txt" || { echo "candidate invalid: nothing written"; exit 1; }
python3 r5-worker-carrying-box-roots.py --apply | tail -1
python3 volumes-insert.py volumes/humans/worker-carrying-box.json "$C" | tail -1
$BUN volumes-check.ts humans/worker-carrying-box --product > "$O/check.txt" 2>&1 || true
grep -o '![a-zA-Z.]*:[a-z-]*=[0-9.]*<=[0-9.]*' "$O/check.txt" || true
tail -1 "$O/check.txt"
$BUN r3-validate.ts humans/worker-carrying-box --volumes 2>&1 | tail -4 | cut -c1-220
