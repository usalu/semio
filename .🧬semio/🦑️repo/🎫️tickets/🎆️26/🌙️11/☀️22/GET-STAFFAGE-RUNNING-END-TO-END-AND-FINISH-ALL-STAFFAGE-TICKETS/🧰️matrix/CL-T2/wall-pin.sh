#!/bin/zsh
# 📌️ Moves the wall pin of `🧪️tests/🧊️model-bounds` (`wall-pin.py`) and runs the four cases of that file that read the brick wall, inside the grant of its caller; the result goes to `$GEN/proof/solid-brick-wall-1960-pin.txt`. wall-pin.sh
source "${0:A:h}/env.sh"
o="$GEN/proof/solid-brick-wall-1960-pin.txt"; mkdir -p "$GEN/vt"; python3 "$W/wall-pin.py" > "$o" 2>&1
(cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️model-bounds/🟦️.ts" -t "bonded masonry") > "$GEN/vt/wall-pin.log" 2>&1
echo "🧊️model-bounds -t 'bonded masonry' exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/wall-pin.log" | tr -s ' ' | tr '\n' ';')" >> "$o"
