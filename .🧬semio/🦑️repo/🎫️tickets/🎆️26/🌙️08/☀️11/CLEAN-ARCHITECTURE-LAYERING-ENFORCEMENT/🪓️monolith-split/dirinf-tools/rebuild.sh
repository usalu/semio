#!/bin/zsh
# regenerate both staged trees from the pristine originals
T=${0:A:h}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws
cd $T
python3 splitdir.py >/dev/null && python3 splitrt.py >/dev/null || exit 1
python3 widen.py $S/widen-dir.json >$S/widen-dir.report 2>&1
python3 widen.py $S/widen-rt.json >$S/widen-rt.report 2>&1
python3 finalize.py $S/stage/dir
python3 finalize.py $S/stage/rt
