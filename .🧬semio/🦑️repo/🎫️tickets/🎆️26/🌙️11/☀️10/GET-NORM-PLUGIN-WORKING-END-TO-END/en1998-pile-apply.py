import sys
S=str(R)+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/schema-evolve"
sys.path.insert(0,S)
import evolve
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
ctx=evolve.build("add-leaf","en1998",["foundations",S+"/specs/en1998-foundation-pile-axial-load.json"],True)
print(len(ctx.plan.files),ctx.unsupported)
if ctx.unsupported: sys.exit("refused")
ctx.plan.commit()
print("committed")
