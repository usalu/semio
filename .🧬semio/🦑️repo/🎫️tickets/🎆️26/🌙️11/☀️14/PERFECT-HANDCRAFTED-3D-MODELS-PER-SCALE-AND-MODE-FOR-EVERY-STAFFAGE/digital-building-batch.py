"""📋 Reopen the next reviewed assets independently and preserve diagnostics for each candidate."""
import subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent
failed=[]
for name in sys.argv[1:]:
    result=subprocess.run([sys.executable,T/'digital-building-readback.py',name])
    if result.returncode:failed.append(name)
print('[DEBUG] unfinished',failed,flush=True)
sys.exit(bool(failed))
