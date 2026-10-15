#!/bin/zsh
# regenerate staged trees, rebuild the sandboxes, run the oracle matrix
T=${0:A:h}
W=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws
$T/rebuild.sh || exit 1
[[ -d $W/sbx-orig ]] || python3 $T/mksbx.py $W/sbx-orig
python3 $T/mksbx.py $W/sbx-split $W/stage/dir $W/stage/rt
$T/matrix.sh $W/sbx-split $W/sbx-orig
