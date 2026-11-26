#!/bin/sh
# 🖨️ Timing of the proof sheets on the classes the first chain did not cover (MM, 2026-11-26): animals, and four trees of the vegetation sheet; one capacity grant of 1500 MB.
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🗑️generated/matrix/MM"; M="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MM"; T="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd /Users/ueli/Documents/semio || exit 1
sh "$M/peak.sh" "$G/h-proofs-animals" timeout 3600 "$BUN" "$T/🖨️proofs.ts" --out "$G/h-proofs2" --categories animals
sh "$M/peak.sh" "$G/h-proofs-trees4" timeout 3600 "$BUN" "$T/🖨️proofs.ts" --out "$G/h-proofs2" --categories vegetation --assets saguaro-cactus,atlas-cedar,olive-tree,small-leaved-lime
