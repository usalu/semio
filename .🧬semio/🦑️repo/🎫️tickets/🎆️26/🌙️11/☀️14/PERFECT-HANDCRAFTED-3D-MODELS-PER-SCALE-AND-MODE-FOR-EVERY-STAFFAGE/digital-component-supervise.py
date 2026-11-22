"""⏱️ Serialize the component readbacks with one native reservation per active asset."""
import os,sys,time,subprocess
from pathlib import Path
T=Path(__file__).resolve().parent
pid=int(sys.argv[1])
while True:
    try:os.kill(pid,0)
    except (OSError,ProcessLookupError):break
    time.sleep(.4)
for name in sys.argv[2:]:
    result=subprocess.run([sys.executable,T/'equipment-guard.py',sys.executable,T/'digital-component-final.py',name])
    print('[DEBUG]',name,'guard',result.returncode,flush=True)
