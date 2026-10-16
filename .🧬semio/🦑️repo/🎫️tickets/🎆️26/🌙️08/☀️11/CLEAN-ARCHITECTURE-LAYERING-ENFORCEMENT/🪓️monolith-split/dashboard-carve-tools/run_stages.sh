#!/bin/zsh
# 🔁 Rebuilds the scratch copy W from the original and applies the carve stages, then runs the union differential.
set -e
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dash
cd "${0:A:h}"
rm -rf $S/W && cp -R $S/orig-dashboard $S/W
python3 carve_a.py $S/W
python3 carve_b.py $S/W
python3 carve_c.py $S/W
python3 carve_d.py $S/W
python3 carve_e.py $S/W $S/orig-dashboard
[ -z "$SKIP_UNION" ] && python3 union.py $S/W ${1:-t}
exit 0
