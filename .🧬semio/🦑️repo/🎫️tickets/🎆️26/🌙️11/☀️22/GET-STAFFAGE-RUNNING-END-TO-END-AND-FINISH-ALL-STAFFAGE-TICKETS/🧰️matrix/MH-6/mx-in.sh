#!/bin/zsh
# 🧮️ The matrix confirmation of one asset inside a grant that is already held (lane.sh; people peak at 290 to 575 MB): lines appended to $GEN/proof/<id>.out: mx-in.sh <id>
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; a="$1"; d="$GEN/proof"
f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; h="$(shasum "$f" | cut -c1-12)"
{
out="$GEN/matrix-$a-$(date +%H%M)"
(cd "$T" && timeout 14400 "$B" 📊️matrix.ts --language en --out "$out" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1); echo "matrix exit $? $(tail -1 "$d/$a-matrix.log") (file $h, $(date '+%d %H:%M'))"
python3 "$M/cells.py" "$out"
echo "file unchanged during the matrix: $([[ "$h" == "$(shasum "$f" | cut -c1-12)" ]] && echo yes || echo NO)"
} >> "$d/$a.out" 2>&1
tail -6 "$d/$a.out"
