#!/bin/zsh
# 🔁️ Trial and light proof of plants of round MV-L3 one after the other inside one grant: `both.sh <id>…` runs `trial.sh` and `light.sh` per plant and prints their lines.
source "${0:A:h}/env.sh"
for a in "$@"; do echo "== $a $(date +%H:%M)"; zsh "$W/trial.sh" "$a" ${=BODY}; zsh "$W/light.sh" "$a"; cat "$GEN/light/$a.txt"; done
