"""🪺 Verify cross-process admission and automatic release after a holder disappears."""
import json
from pathlib import Path
import runpy,subprocess,sys,tempfile
t=Path(__file__).resolve().parent
case=json.loads((t/'digital-budget-case.json').read_text());mod=t/'equipment-guard.py'
worker="import sys,runpy;p=runpy.run_path(sys.argv[1]);s=p['reservation'](sys.argv[2],int(sys.argv[3]),int(sys.argv[4]));print(int(s is not None),flush=True);sys.stdin.readline()"
def start(path,mib,capacity=case['capacityMiB']):
    p=subprocess.Popen([sys.executable,'-c',worker,str(mod),str(path),str(mib),str(capacity)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
    return p,p.stdout.readline().strip()
with tempfile.TemporaryDirectory(dir=t/'🗑️generated') as d:
    path=Path(d)/'lock';procs=[]
    try:
        for mib,wanted in zip(case['reservationsMiB'],case['admitted']):
            p,admitted=start(path,mib);procs.append(p);assert admitted==str(int(wanted)),(mib,admitted)
        procs[0].kill();procs[0].wait()
        p,admitted=start(path,case['afterHolderExitsMiB']);procs.append(p);assert admitted=='1'
    finally:
        for p in procs:p.stdin.close();p.wait()
print('Weighted reservations bounded concurrent jobs and the OS reclaimed a terminated holder.')

fragment=case['fragmentation']
with tempfile.TemporaryDirectory(dir=t/'🗑️generated') as d:
    path=Path(d)/'lock';procs=[]
    try:
        for mib in fragment['holdersMiB']:
            child,admitted=start(path,mib,fragment['capacityMiB']);procs.append(child);assert admitted=='1'
        for i in fragment['release']:
            procs[i].stdin.close();procs[i].wait()
        child,admitted=start(path,fragment['admitMiB'],fragment['capacityMiB']);procs.append(child)
        assert admitted=='1','Two separated free slots must admit a two-slot job.'
    finally:
        for child in procs:
            if not child.stdin.closed:child.stdin.close()
            child.wait()
print('Reused separated free slots without exceeding the host budget.')
