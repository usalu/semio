#!/bin/zsh
# ⛓️ Light proof, then write and proof, asset by asset inside the grant of its caller (`prove.sh` refuses an asset whose light proof did not pass): chain.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do zsh "$W/light.sh" "$a"; tail -1 "$GEN/light/$a.txt"; zsh "$W/prove.sh" "$a"; done
