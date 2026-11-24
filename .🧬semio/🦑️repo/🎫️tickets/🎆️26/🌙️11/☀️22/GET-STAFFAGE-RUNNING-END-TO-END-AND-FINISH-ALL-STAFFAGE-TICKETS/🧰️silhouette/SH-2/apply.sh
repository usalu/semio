#!/bin/zsh
# 🧷️ Applies the repair ops of people to their catalog files: the TypeScript applier for the people of HW-10 … HW-18 (plainly serialised files), the applier of the chain for `tennis-player`, the span-preserving Python applier for the hand-laid-out files. apply.sh <slug>…
source "${0:A:h}/env.sh"; cd "$W"
for a in "$@"; do
  if grep -q "\"$a\": \"HW-1[0-9]\"" people.ts; then "$B" apply.ts "$a" | tail -1; elif [[ "$a" == tennis-player ]]; then python3 apply_chain.py "$a" | tail -1; else python3 apply.py "$a" | tail -1; fi
done
