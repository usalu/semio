#!/bin/zsh
# 📐️ The state of assets before this lane writes them, inside the grant of its caller: base.sh <id>… writes the triangles by group (`groups.ts`) to `$GEN/base/<id>-groups.txt`, the GLB of the default pose by mesh group (`own2.ts`) to `$GEN/base/<id>-glb.txt` and the GLB of every pose (`sizes.sh`) to `$GEN/sizes.txt`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/base"
for a in "$@"; do
  zsh "$W/run.sh" groups.ts "$a" --top 14 > "$GEN/base/$a-groups.txt" 2>&1
  zsh "$W/run.sh" own2.ts "$a" --top 40 > "$GEN/base/$a-glb.txt" 2>&1
  zsh "$W/sizes.sh" "$a" > "$GEN/base/$a-sizes.txt"
  grep -E "^$a |^BUDGET" "$GEN/base/$a-groups.txt"; head -1 "$GEN/base/$a-glb.txt"; cat "$GEN/base/$a-sizes.txt"
done
