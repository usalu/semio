#!/bin/zsh
# ⛓️ The whole proof of one asset of lane CL-P inside one grant: the light proof of the trial copy, then (only after PASS) the write with the post-write proof, then the real `🧊️model-catalog` with the asset as the only selected one. chain.sh <id>
source "${0:A:h}/env.sh"
zsh "$W/light.sh" "$1"; tail -1 "$GEN/light/$1.txt"
grep -q "LIGHT PROOF $1: PASS" "$GEN/light/$1.txt" || exit 1
zsh "$W/prove.sh" "$1" || exit 1
zsh "$W/modelcat.sh" "$1" "$1"; tail -1 "$GEN/proof/model-catalog.txt"
