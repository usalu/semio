import sys; sys.path.insert(0,'.')
from wgen import *
SCRO = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
OUT = SCRO + '/stage-w2'
P = plan(); crates, ic, imports = build_crates(P)
shutil.rmtree(OUT, ignore_errors=True)
shutil.copytree(SCRO + '/stage-w1', OUT, symlinks=True)
# the old root world file and test dirs stay for now (tests handled separately)
write_crates(crates, OUT)
for c in crates: print(c.id, len(c.text.split('\n')))
