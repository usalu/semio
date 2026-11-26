#!/bin/sh
# 🔗️ The two closing request chains of MM (22:40): chain-final.sh <1|2>; each runs inside one capacity grant (1: 1600 MB, 2: 2500 MB), the logs are <generated>/h-*.log.
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🗑️generated/matrix/MM"; M="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MM"; T="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd /Users/ueli/Documents/semio || exit 1
if [ "$1" = 1 ]; then
  LIMIT_S=14400 sh "$M/timed.sh" "$G/h-trees" atlas-cedar olive-tree
  LIMIT_S=28800 sh "$M/timed.sh" "$G/h-cyclist" cyclist
else
  LIMIT_S=14400 sh "$M/timed.sh" "$G/h-lime" small-leaved-lime
  LIMIT_S=28800 sh "$M/timed.sh" "$G/h-dancer" dancer
  for c in machinery furniture; do sh "$M/peak.sh" "$G/h-proofs-$c" timeout 3600 "$BUN" "$T/🖨️proofs.ts" --out "$G/h-proofs" --categories "$c"; done
fi
