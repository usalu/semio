#!/bin/zsh
# 📏️ Measures assets of batch MC in one queue grant, one process after the other: the evaluated boxes of every shape in every pose (`boxes.ts`, into $GEN/boxes-<tag>), the fixer's gate over every subject of the matrix (`../MR/gate.ts`: poses, clip instants, transitions) and the authors' gate of the suite (`T/🔎️scan.ts`), on the catalog documents or, with SRC=<dir>, on the copies `<dir>/<id>.json` (gate only). Logs: $GEN/log/<tag>-gate.txt, <tag>-scan.txt, <tag>-scan.json. measure.sh <tag> [<id>…]   (run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
tag="$1"; shift; ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
mkdir -p "$GEN/log"; cd "$T"
: > "$GEN/log/$tag-gate.txt"
srcs=(); for a in $ids; do srcs+=("${SRC:+$SRC/}$a${SRC:+.json}"); done
timeout 1500 "$B" "$W/boxes.ts" "$GEN/boxes-$tag" $srcs > "$GEN/log/$tag-boxes.txt" 2>&1
for a in $ids; do
  timeout 1500 "$B" "$M/gate.ts" "${SRC:+$SRC/}$a${SRC:+.json}" --soft >> "$GEN/log/$tag-gate.txt" 2>&1
  echo "[DEBUG] gate $a exit $?" >> "$GEN/log/$tag-gate.txt"
done
[[ -n "$SRC" ]] && exit 0
timeout 3000 "$B" 🔎️scan.ts --language en --fresh --out "$GEN/log/$tag-scan.jsonl" --report "$GEN/log/$tag-scan.json" $ids > "$GEN/log/$tag-scan.txt" 2>&1
echo "[DEBUG] scan exit $?" >> "$GEN/log/$tag-scan.txt"
