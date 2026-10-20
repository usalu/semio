#!/bin/zsh
# 🔬 Runs the (scratch or live) library inventory for the scopes in a list file, N in parallel. usage: [TAX5_SIM=tax5-sim] [TAX5_LIVE=1] run.sh <listfile> <outdir> <parallel>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
mkdir -p "$2"
one() {
  scope="$1"
  name=$(printf '%s' "$scope" | command sed 's#^✏️s/##; s#/#__#g')
  start=$(date +%s)
  perl -e 'alarm 570; exec @ARGV' -- bun "$T/tax5-tools/sim-inventory.ts" "$scope" "$2/inv-$name.json" > "$2/inv-$name.log" 2>&1
  echo "$scope rc=$? secs=$(( $(date +%s) - start )) $(command tail -1 "$2/inv-$name.log")"
}
while IFS= read -r scope; do
  one "$scope" "$2" &
  while [ "$(jobs -rp | wc -l)" -ge "$3" ]; do sleep 2; done
done < "$1"
wait
