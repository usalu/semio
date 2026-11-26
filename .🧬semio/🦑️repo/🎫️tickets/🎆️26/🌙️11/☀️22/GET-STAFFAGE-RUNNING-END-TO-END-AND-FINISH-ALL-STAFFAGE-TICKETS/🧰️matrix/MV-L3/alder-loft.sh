#!/bin/zsh
# 🌲️ The loft copy of `black-alder` through its light proof, write and proof inside one grant, for the moment the pocket rule of `🧊️catalog-silhouette` is live: the light proof of `$GEN/loft/black-alder.json` (with the place of every failing view, `light-loft.sh`); on PASS the copy becomes `$GEN/try/black-alder.json`, the method `loft`, and `prove.sh` writes and proves; on FAIL nothing is written.
source "${0:A:h}/env.sh"
a=black-alder
zsh "$W/light-loft.sh" "$a"
grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a-loft.txt" || { echo "$a: loft copy fails the light proof, nothing written"; exit 1; }
cp "$GEN/loft/$a.json" "$GEN/try/$a.json"; cp "$GEN/light/$a-loft.txt" "$GEN/light/$a.txt"; echo loft > "$W/flags/$a.method"
zsh "$W/prove.sh" "$a"; cat "$GEN/proof/$a.txt"
