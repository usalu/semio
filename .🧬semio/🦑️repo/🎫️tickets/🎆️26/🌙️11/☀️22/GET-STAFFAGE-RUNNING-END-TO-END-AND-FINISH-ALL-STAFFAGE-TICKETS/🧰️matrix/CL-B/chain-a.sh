#!/bin/zsh
# 🧮️ Chain A of lane CL-B, one queue request after the other: the red of the former arrow outline (`red.ts`), the matrix of the chairs and of four further assets on the engine as it is (`--fresh --jobs 1`, 800 MB), then the small suites that draw forces, arrows or section markers in one vitest run (2500 MB). Lines into `$GEN/log/runs.txt`, the last line of `$GEN/log/chain-a.done` marks the end.
source "${0:A:h}/env.sh"
zsh "$W/q.sh" 800 300 red "$T" "$B" "$W/red.ts"
rm -rf "$GEN/matrix"
zsh "$W/q.sh" 800 7200 matrix "$T" "$B" 📊️matrix.ts --language en --out "$GEN/matrix" --jobs 1 --fresh --assets used-bentwood-dining-chairs,cafe-chair,park-bench,used-timpani-pair,jogger
python3 "$M/cells.py" "$GEN/matrix" > "$GEN/log/matrix-cells.txt" 2>&1
zsh "$W/q.sh" 2500 3600 small "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🏗️structure-projection/🟦️.ts" "../../🧪️tests/✂️section-views/🟦️.ts" "../../🧪️tests/🎨️seven-modes/🟦️.ts" "../../🧪️tests/🎨️view-synchrony/🟦️.ts" "../../🧪️tests/📑️multiview-sheet/🟦️.ts" "../../🧪️tests/🤐️textless-depiction/🟦️.ts" "../../🧪️tests/🎨️structure-legend/🟦️.ts" "../../🧪️tests/🥇️force-first/🟦️.ts" "../../🧪️tests/📐️dxf-reopen/🟦️.ts" "../../🧪️tests/⚖️equilibrium/🟦️.ts"
date > "$GEN/log/chain-a.done"
