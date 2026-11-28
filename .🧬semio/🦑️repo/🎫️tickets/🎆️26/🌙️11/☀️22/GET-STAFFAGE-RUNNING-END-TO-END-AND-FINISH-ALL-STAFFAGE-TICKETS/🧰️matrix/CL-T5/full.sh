#!/bin/zsh
# 🏁️ The whole way of one convex-lobed tree inside one grant: full.sh <id> <segments> builds the ops from the crown analysis of this moment with its trial numbers (`tree.sh`) and then runs `chain.sh` (trial copy, light proof, GLB of every pose within 7.5 MiB, and only then the write with the post-write proof, the matrix and the single-asset `🧊️model-catalog` run).
source "${0:A:h}/env.sh"
zsh "$W/tree.sh" "$1" "$2" && zsh "$W/chain.sh" "$1"
