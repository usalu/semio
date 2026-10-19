#!/bin/zsh
# 🔬 Runs sim-inventory.ts for each scope in a list file slice. usage: sim-run.sh <listfile> <from> <to> <outdir> [bun script]
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
script="${5:-sim-inventory.ts}"
sed -n "$2,$3p" "$1" | while IFS= read -r scope; do
  name=$(printf '%s' "$scope" | command sed 's#^✏️s/##; s#/#__#g')
  start=$(date +%s)
  perl -e 'alarm 280; exec @ARGV' -- bun "$T/tax3-tools/$script" "$scope" "$4/sim-$name.json" > "$4/sim-$name.log" 2>&1
  echo "$scope rc=$? secs=$(( $(date +%s) - start )) $(command tail -1 "$4/sim-$name.log")"
done
