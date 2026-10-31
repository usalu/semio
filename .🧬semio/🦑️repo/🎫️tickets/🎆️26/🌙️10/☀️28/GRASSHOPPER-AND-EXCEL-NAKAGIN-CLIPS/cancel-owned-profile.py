"""🛑 Cancel only the explicitly identified superseded renderer profile."""
import json
import os
from pathlib import Path
import signal
import sys
root=Path(__file__).resolve().parent/'🗑️generated'
path=root/'renderer-profile-journal.json'
record=json.loads(path.read_text(encoding='utf-8'))
pid=int(sys.argv[1])
if pid!=25336 or record['runtime']['pid']!=pid or record['state']!='running':
    raise ValueError('Owned profile identity no longer matches')
(root/'renderer-profile-journal-cancelled.json').write_text(json.dumps({'state':'cancelled','reason':'Superseded by proven common-plane/singleton correction','pid':pid,'last_snapshot':record},indent=2)+'\n',encoding='utf-8')
os.kill(pid,signal.SIGTERM)
print('[DEBUG] Cancelled only superseded owned profile PID',pid)
