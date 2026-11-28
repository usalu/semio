#!/bin/zsh
# 🧪️ A trial copy and its numbers inside the grant of its caller: trial.sh <id> makes `$GEN/try/<id>.json` from `ops/<id>.json` (`make.sh`), then the triangles (`groups.ts`) into `$GEN/trial/<id>-groups.txt` and the GLB of the default pose by mesh group (`own2.ts`) into `$GEN/trial/<id>-glb.txt`.
source "${0:A:h}/env.sh"
a="$1"; mkdir -p "$GEN/trial"
zsh "$W/make.sh" "$a" > "$GEN/trial/$a-make.txt" 2>&1 || { tail -3 "$GEN/trial/$a-make.txt"; exit 1; }
zsh "$W/run.sh" groups.ts "$GEN/try/$a.json" --top 12 > "$GEN/trial/$a-groups.txt" 2>&1
zsh "$W/run.sh" own2.ts "$GEN/try/$a.json" --top 40 "${@:2}" > "$GEN/trial/$a-glb.txt" 2>&1
grep -E "^$a (body|thermal|structure):|^BUDGET|^  !" "$GEN/trial/$a-groups.txt" | cut -c1-200; head -1 "$GEN/trial/$a-glb.txt" | cut -c1-200
