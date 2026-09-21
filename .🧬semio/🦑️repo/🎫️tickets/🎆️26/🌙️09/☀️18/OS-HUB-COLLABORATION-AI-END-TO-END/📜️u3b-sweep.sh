#!/bin/zsh
# 🧪️ U3b: batched native `cargo check` over every crate the U3 codemod touched.
# Usage: zsh 📜️u3b-sweep.sh <first-batch-index> <last-batch-index>
set -u
cd "$(dirname "$0")/../../../../../../.." || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
BATCHES="$T/🗑️generated/u3b-batches.json"
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-u3b"
for i in $(seq "$1" "$2"); do
  args=$(python3 -c "
import json,sys
b=json.load(open(sys.argv[1]))[int(sys.argv[2])-1]
print(' '.join('-p '+c for c in b['crates']) + (' --features component-app-assembly' if b['feat'] else ''))
" "$BATCHES" "$i")
  out="$T/🗑️generated/u3b-sweep-batch$i.txt"
  echo "### batch $i: cargo check ${args} --keep-going" | tee "$out"
  eval cargo check ${args} --keep-going >> "$out" 2>&1
  echo "EXIT=$? batch=$i errors=$(grep -c '^error' "$out") warnings=$(grep -c '^warning' "$out")"
done
