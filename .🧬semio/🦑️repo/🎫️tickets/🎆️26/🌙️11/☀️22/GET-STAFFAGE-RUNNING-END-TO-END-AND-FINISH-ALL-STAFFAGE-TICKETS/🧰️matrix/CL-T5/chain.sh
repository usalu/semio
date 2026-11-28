#!/bin/zsh
# ⛓️ The whole way of assets of lane CL-T5 inside one grant, one asset after the other: a fresh trial copy from the catalog file of this moment (`make.sh`), the light proof of the copy (`light2.sh`), the GLB of every pose of the copy (`sizes.sh`, must be within 7.5 MiB), and only when both passed the write with the proof (`prove2.sh`). chain.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
  zsh "$W/make.sh" "$a" | tail -1; [[ -s "$GEN/try/$a.json" ]] || continue
  zsh "$W/light2.sh" "$a"; tail -1 "$GEN/light/$a.txt"
  zsh "$W/sizes.sh" "$GEN/try/$a.json" | tee "$GEN/light/$a-sizes.txt"
  grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a.txt" && grep -q " ok " "$GEN/light/$a-sizes.txt" && zsh "$W/prove2.sh" "$a" && tail -4 "$GEN/proof/$a.txt" | cut -c1-300
done
