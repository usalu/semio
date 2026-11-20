#!/bin/sh
# 🚢 Builds the round-3 date palm, validates the candidate, writes the root fixes and splices the volumes into the catalog, then validates the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/date-palm/🔣️.json"
CAND="🗑️generated/r3-date-palm/🔣️.json"
$BUN r3-date-palm-model.ts --source > "🗑️generated/r3-date-palm/model.txt"; tail -6 "🗑️generated/r3-date-palm/model.txt"
$BUN r3-validate.ts --file "$CAND" > "🗑️generated/r3-date-palm/validate.txt" || { tail -8 "🗑️generated/r3-date-palm/validate.txt"; exit 1; }
tail -1 "🗑️generated/r3-date-palm/validate.txt"
python3 r3-date-palm-roots.py "$CAND" "$CAT" ${MEMBERS:-rig anchors summary provenance poses clips thermal representations}
python3 volumes-insert.py volumes/vegetation/date-palm.json "$CAT"
$BUN r3-validate.ts vegetation/date-palm --catalog-only | tail -3
