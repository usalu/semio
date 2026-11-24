#!/bin/zsh
# 🩹️ Repair ops of `wheel-loader`: the declared dimensions where they missed the body (declared-dimensions.py), then the parts of the volume that no view drew, at every scale and view (patch-views.sh); added parts that would make a scale unprintable (a gap or feature below the minimum of the print profile) are taken out again (printable.ts). Checks stay red, see the report.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 declared-dimensions.py >/dev/null; cd "$W"
[[ -f ops/wheel-loader.json ]] || echo '[]' > ops/wheel-loader.json
author/patch-views.sh wheel-loader

"$B" printable.ts wheel-loader
