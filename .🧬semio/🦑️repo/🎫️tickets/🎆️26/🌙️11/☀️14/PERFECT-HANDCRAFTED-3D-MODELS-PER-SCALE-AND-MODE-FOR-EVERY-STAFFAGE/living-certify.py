"""🌿 Preserve a readable summary of the actual current-source verification evidence."""
import hashlib
import json
import subprocess
import sys
from datetime import datetime
from pathlib import Path
ticket=Path(__file__).parent
out=ticket/'🗑️generated/living-current'
root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
pending={(line.split()[0],line.split()[1]) for line in subprocess.check_output([sys.executable,str(ticket/'living-batch.py'),'--bun',sys.argv[1],'--list'],text=True).splitlines() if line}
report=['# Plant and animal model verification', '',f"Recorded on {datetime.now().astimezone().isoformat(timespec='minutes')}. Each row covers body, thermal and structural models at 1:50, 1:100, 1:200 and 1:500. All 30 passed the per-solid validator, rig and region associations, printable features, slenderness and the product’s cross-scale geometry-copy check on the hashes listed below. The geometric scene budget is 400,000 triangles and the presentation file limit is 8,388,608 bytes.",'', 'The strict column includes silhouette agreement, physical bounds, union volume and exposed area. The export column includes the independent GLB reload. A preceding-code measurement used the same catalog geometry. The checkpointed audit repeats it when relevant product code changes. Earlier results without a checkpointed code signature are shown only where the size budget was explicitly measured.', '', '| Asset | Catalog SHA-256 prefix | Strict volume result | Measured presentation GLB |', '|---|---|---|---|']
def load(name):
    p=out/name
    return json.loads(p.read_text()) if p.exists() else {}
counts={'strict':0,'glb':0,'complete':0,'strict-source':0,'glb-source':0}
for f in sorted((*root.glob('*animals/*/*.json'),*root.glob('*vegetation/*/*.json')),key=lambda p:p.parent.name):
    name=f.parent.name;sha=hashlib.sha256(f.read_bytes()).hexdigest()
    a=load(name+'-strict.json');b=load(name+'-glb.json')
    src=load(name+'-source.json')
    assert (src.get('hash')==sha and src.get('findings')==[]) or all((lambda v:v.get('hash')==sha and v.get('findings')==[])(load(name+'-source-'+k+'.json')) for k in ('50','100','200','500')),name
    copied=load(name+'-copied.json');assert copied.get('hash')==sha and copied.get('findings')==[],name
    def state(mode,f):
        if f.get('hash')!=sha:return 'Queued'
        if mode=='glb' and f.get('payload') is None:return 'Queued'
        if mode=='glb' and f.get('bytes',2**32)>8388608:return f"{f['bytes']:,} bytes, over limit"
        current=(name,mode) not in pending
        if f.get('findings'):return str(len(f['findings']))+' finding(s)'+('' if current else ' on preceding code')
        counts[mode+'-source']+=1
        if current:counts[mode]+=1
        if mode=='strict':return 'Pass' if current else 'Pass on preceding code'
        return f"{f['bytes']:,} bytes"+('' if current else ', preceding code')
    sr,gr=state('strict',a),state('glb',b)
    if sr=='Pass' and gr.endswith('bytes'):counts['complete']+=1
    report.append(f'| {name} | `{sha[:12]}` | {sr} | {gr} |')
report+=['',f"At this checkpoint, the exact catalog documents in the table have {counts['strict-source']} complete strict passes and {counts['glb-source']} budgeted GLB passes. Of these, {counts['strict']} strict and {counts['glb']} GLB checks also match the current test and product code signature, including {counts['complete']} that match in both columns. All file sizes come from the presentation-pose GLB binary. The source-only triangle and binding checks are recorded in [the living model audit](living-current-audit.md).",'']
(ticket/'living-certification.md').write_text('\n'.join(report))

