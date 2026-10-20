#!/bin/zsh
# 🔬 Runs the live-library inventory for the scopes listed in a file slice, N in parallel. usage: run.sh <listfile> <from> <to> <parallel>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OUT="$T/🗑️generated/${TAX4_OUT:-tax4-inv}"
cd /Users/ueli/Documents/semio
one() {
  scope="$1"
  name=$(printf '%s' "$scope" | command sed 's#^✏️s/##; s#/#__#g')
  [ "$scope" = "." ] && name="ROOT"
  start=$(date +%s)
  perl -e 'alarm 570; exec @ARGV' -- bun "$T/tax4-tools/${TAX4_SCRIPT:-inventory.ts}" "$scope" "$OUT/inv-$name.json" > "$OUT/inv-$name.log" 2>&1
  echo "$scope rc=$? secs=$(( $(date +%s) - start )) $(command tail -1 "$OUT/inv-$name.log")"
}
sed -n "$2,$3p" "$1" | while IFS= read -r scope; do
  one "$scope" &
  while [ "$(jobs -rp | wc -l)" -ge "$4" ]; do sleep 2; done
done
wait
