#!/bin/zsh
# 🧪️ U5: batched native `cargo check` over every crate the U5 label pass touched, one cargo at a time.
# Usage: nohup zsh u5-check-sweep.sh <first-batch> <last-batch> > <log> 2>&1 & disown
set -u
cd /Users/ueli/Documents/semio || exit 1
G=/Users/ueli/Documents/semio/.tmp-ticket/wp-u5/generated
export CARGO_INCREMENTAL=0
for i in $(seq "$1" "$2"); do
  args=$(python3 -c "
import json,sys
print(' '.join('-p '+c for c in json.load(open(sys.argv[1]))[int(sys.argv[2])-1]))
" "$G/u5-check-batches.json" "$i")
  out="$G/u5-check-batch$i.txt"
  echo "### batch $i $(date +%T): cargo check ${args} --keep-going" > "$out"
  eval cargo check ${args} --keep-going --message-format short >> "$out" 2>&1
  rc=$?
  echo "EXIT=$rc batch=$i $(date +%T) errors=$(/usr/bin/grep -c 'error' "$out") warnings=$(/usr/bin/grep -c 'warning' "$out")"
done
echo "SWEEP-DONE $(date +%T)"
