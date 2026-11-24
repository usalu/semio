#!/bin/zsh
# 🩹️ Repair ops of `concrete-mixer-truck`: the declared dimensions where they missed the body (declared-dimensions.py), then the parts of the volume that no view drew, at every scale and view (patch-views.sh); added parts that would make a scale unprintable (a gap or feature below the minimum of the print profile) are taken out again (printable.ts); the two chute hinges are left out again in the plan of 1:50, where the validator refuses a flat shape on the tilted chute. Checks stay red, see the report.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 declared-dimensions.py >/dev/null; cd "$W"
[[ -f ops/concrete-mixer-truck.json ]] || echo '[]' > ops/concrete-mixer-truck.json
author/patch-views.sh concrete-mixer-truck
python3 author/drop-shapes.py concrete-mixer-truck chute-hinge
"$B" printable.ts concrete-mixer-truck
