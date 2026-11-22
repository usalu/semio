#!/bin/zsh
# 🩹 Runs the silhouette guard's greedy trace repair (humansb-D-auto.ts, imported from its ticket) for one asset at the given scales and
# writes the ops to ops/<asset>-<scale>.json. Usage: repair.sh <asset> <scale>...
source "${0:A:h}/env.sh"
A="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/humansb-D-auto.ts"
cd "$PK"
a=$1; shift
for s in "$@"; do
  $B "$A" $a $s "$W/ops/$a-$s.json" 40 > "$GEN/[DEBUG]-repair-$a-$s.log" 2>&1
  echo "$a $s $(grep final "$GEN/[DEBUG]-repair-$a-$s.log")"
done
