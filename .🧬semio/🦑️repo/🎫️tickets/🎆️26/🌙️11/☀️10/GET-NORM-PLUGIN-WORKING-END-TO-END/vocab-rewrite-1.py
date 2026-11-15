import re,sys
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
subs=[(r"`NoMutation` (?:was|is) dropped: the ","The "),(r"`NoMutation` (?:was|is) dropped: ","")]
for l in open(sys.argv[1],encoding="utf-8"):
    f=l.rstrip("\n").split("\t")[1]
    t=open(str(R)+"/"+f,encoding="utf-8").read();o=t
    for a,b in subs:t=re.sub(a,b,t)
    if t!=o:open(str(R)+"/"+f,"w",encoding="utf-8").write(t)
