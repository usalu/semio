#!/bin/zsh
# ⛓️ The whole way of assets of lane CL-T1 inside one grant, one asset after the other: a fresh trial copy from the catalog file of this moment (`make.sh`), the light proof of the copy (`light.sh`), and only when it passed the write with the proof (`prove.sh`). chain.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
  zsh "$W/make.sh" "$a" | tail -1
  zsh "$W/light.sh" "$a"; tail -1 "$GEN/light/$a.txt"
  grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a.txt" && zsh "$W/prove.sh" "$a" && tail -4 "$GEN/proof/$a.txt" | cut -c1-300
done
