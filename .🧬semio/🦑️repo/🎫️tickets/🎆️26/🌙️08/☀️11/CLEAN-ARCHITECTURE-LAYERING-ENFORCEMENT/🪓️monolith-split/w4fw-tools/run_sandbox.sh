#!/bin/zsh
# rebuild the work sandbox from base and apply the cut
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad
rm -rf $S/sb/work; cp -R $S/sb/base $S/sb/work
cd "${0:A:h}" && python3 - <<PY
import sys
sys.path.insert(0,'.')
from apply import *
S="$S/sb/work/"
cut=Cut(S); b,rw=apply(cut); texts=write_out(cut,b,rw)
print(rw.log)
PY
