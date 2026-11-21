"""🪺 Exercise normal exit and cancellation with a sleeping descendant of a guarded command."""
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

ticket=Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(dir=ticket/'🗑️generated') as temp:
    for finish in (True,False):
        stamp=Path(temp)/str(finish)
        worker="import time;time.sleep(30)"
        script="import subprocess,sys,time;from pathlib import Path;p=subprocess.Popen([sys.executable,'-c',sys.argv[1]]);Path(sys.argv[2]).write_text(str(p.pid));time.sleep(float(sys.argv[3]))"
        guard=subprocess.Popen([sys.executable,str(ticket/'equipment-guard.py'),sys.executable,'-c',script,worker,str(stamp),'0.01' if finish else '30'],env={**os.environ,'EQUIPMENT_RSS_MIB':'64'})
        try:
            until=time.monotonic()+180
            while not (stamp.exists() and stamp.read_text()):
                if guard.poll() is not None:raise RuntimeError(f'Guard exited before test worker startup: {guard.returncode}')
                if time.monotonic()>until:raise TimeoutError('Host pool did not admit cleanup probe')
                time.sleep(.05)
            pid=int(stamp.read_text())
            if not finish:guard.terminate()
            guard.wait(timeout=5)
            if os.name=='nt':
                output=subprocess.check_output(['tasklist','/FI',f'PID eq {pid}','/NH'],text=True)
                alive=str(pid) in output
            else:
                state=subprocess.run(['ps','-o','stat=','-p',str(pid)],capture_output=True,text=True).stdout.strip()
                alive=bool(state and not state.startswith('Z'))
            assert not alive, f'child {pid} survived guarded job'
        finally:
            if guard.poll() is None:guard.terminate();guard.wait(timeout=5)
            if stamp.exists() and stamp.read_text():
                try:os.kill(int(stamp.read_text()),signal.SIGTERM)
                except OSError:pass
    data="import time;a=bytearray(24*1024*1024);time.sleep(5)"
    tree="import subprocess,sys;p=[subprocess.Popen([sys.executable,'-c',sys.argv[1]]) for i in range(2)];sys.exit(any([x.wait() for x in p]))"
    r=subprocess.run([sys.executable,str(ticket/'equipment-guard.py'),sys.executable,'-c',tree,data],env={**os.environ,'EQUIPMENT_RSS_MIB':'55'},capture_output=True,text=True,timeout=180)
    assert r.returncode!=0 and ('paused for headroom' in r.stderr or os.name=='nt'),r
print('Guard reaped descendants on completion and cancellation and enforced the aggregate job budget.')
