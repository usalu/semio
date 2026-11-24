#!/usr/bin/env bash
# 🧭️ Paths of batch MB-2: source it from bash.
R=/Users/ueli/Documents/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$S/🧪️tests/🔭️multiview-coverage"
U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
M="$U/🧰️matrix/MB-2"
G="$U/🗑️generated/matrix/MB-2"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
IDS=$(cut -f2 "$U/🧰️matrix/batches/MB-2.tsv")
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
