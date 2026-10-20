#!/bin/zsh
# 🔬 Runs the live-library inventory for the scopes listed in a file, N in parallel. usage: TAX6_OUT=<dir> run.sh <listfile> <parallel>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OUT="$T/🗑️generated/tax6/${TAX6_OUT:-inv}"
mkdir -p "$OUT"
cd /Users/ueli/Documents/semio
one() {
  scope="$1"
  name=$(printf '%s' "$scope" | command sed 's#/#__#g')
  start=$(date +%s)
  perl -e 'alarm 580; exec @ARGV' -- bun --tsconfig-override "$T/tax6-tools/tsconfig-shim.json" "$T/tax6-tools/${TAX6_SCRIPT:-inventory.ts}" "$scope" "$OUT/inv-$name.json" > "$OUT/inv-$name.log" 2>&1
  echo "$scope rc=$? secs=$(( $(date +%s) - start )) $(command tail -1 "$OUT/inv-$name.log")"
}
while IFS= read -r scope; do
  one "$scope" &
  while [ "$(jobs -rp | wc -l)" -ge "$2" ]; do sleep 2; done
done < "$1"
wait
