#!/bin/zsh
# 🪶️ The trial copy, its light proof and its GLB sizes without the write: try.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
  zsh "$W/make.sh" "$a" | tail -1; [[ -s "$GEN/try/$a.json" ]] || continue
  zsh "$W/light2.sh" "$a"; cat "$GEN/light/$a.txt" | cut -c1-260
  zsh "$W/sizes.sh" "$GEN/try/$a.json" | tee "$GEN/light/$a-sizes.txt"
done
