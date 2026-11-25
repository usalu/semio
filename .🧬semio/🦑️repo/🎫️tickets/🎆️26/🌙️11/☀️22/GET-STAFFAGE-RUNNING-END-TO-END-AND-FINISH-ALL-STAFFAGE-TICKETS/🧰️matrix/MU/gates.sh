#!/bin/zsh
# 🚦️ The fixer's gate (`../MR/gate.ts`, poses, clip instants and transitions) for assets one process after the other, one log per asset in $GEN/gate/<id>.txt, finished assets are skipped unless FRESH=1: gates.sh <id>…   (run it through q.sh)
source "${0:h}/env.sh"
mkdir -p "$GEN/gate"
cd "$T"
for a in "$@"; do
  [[ -z "$FRESH" ]] && grep -q '^hard findings' "$GEN/gate/$a.txt" 2>/dev/null && continue
  timeout ${GATE_TIMEOUT:-3600} "$B" "$M/gate.ts" "$a" > "$GEN/gate/$a.txt" 2>&1
  echo "$a exit $? $(grep '^hard findings' "$GEN/gate/$a.txt")"
done
