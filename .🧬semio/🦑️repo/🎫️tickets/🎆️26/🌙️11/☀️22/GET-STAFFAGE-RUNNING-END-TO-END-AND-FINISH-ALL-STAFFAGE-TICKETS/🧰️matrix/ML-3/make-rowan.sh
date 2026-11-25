#!/bin/zsh
# 🏭️ The repair of `rowan` of round ML-3, the last step of its chain (generator, silhouette ops, then `../MV-3/make.sh rowan`: 1:500 crown, depth plates, anchors; `../MS/make-rowan.sh` is no longer part of the chain), idempotent, for a job inside a grant (`steps.sh`): `base-rowan-200.ts --write` sets the 1:200 front and plan back to one flat outline per part (`ops/rowan-200-base.json`), then `../MV-L1/loft.ts rowan --ops ops/rowan.json --write` removes every depth plate (`mvc-*`) and every sheet suffix `-mvs` and sets the front and plan outline of every foliage mass at 1:50, 1:100 and 1:200 as the loft of its own ellipsoids, the absolute values `search-rowan.sh` saved. Prints the digest before and after. make-rowan.sh
source "${0:A:h}/steps.sh"
f="$C/rowan/🔣️.json"; b="$(shasum -a 256 "$f" | cut -c1-16)"
tool rowan-make-base base-rowan-200.ts --write
tool rowan-make-loft "$U/🧰️matrix/MV-L1/loft.ts" rowan --ops "$W/ops/rowan.json" --write
echo "rowan catalog file $b -> $(shasum -a 256 "$f" | cut -c1-16) $(date +%H:%M)"
