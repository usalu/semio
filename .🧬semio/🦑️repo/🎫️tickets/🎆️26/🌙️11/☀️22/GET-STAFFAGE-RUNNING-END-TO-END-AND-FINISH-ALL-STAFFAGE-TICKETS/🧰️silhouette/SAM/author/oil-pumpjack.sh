#!/bin/zsh
# 🩹️ Repair ops of `oil-pumpjack`: the declared dimensions where they missed the body (declared-dimensions.py), then the parts of the volume that no view drew, at every scale and view (patch-views.sh); added parts that would make a scale unprintable (a gap or feature below the minimum of the print profile) are taken out again (printable.ts). Checks stay red, see the report.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 declared-dimensions.py >/dev/null; cd "$W"
[[ -f ops/oil-pumpjack.json ]] || echo '[]' > ops/oil-pumpjack.json
author/patch-views.sh oil-pumpjack

"$B" printable.ts oil-pumpjack
