"""🧾 Track measured whole-asset volume checks by source revision for all 152 staffage assets."""
from collections import Counter
from datetime import datetime,timezone
from hashlib import sha256
from pathlib import Path
import json,os,shutil,subprocess,sys
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';fleets=g/'equipment-fleet';living=g/'living-current'
def read(p):return json.loads(p.read_bytes()) if p.exists() else {}
bun=os.getenv('BUN_EXE') or shutil.which('bun') or next(p for p in r.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
livingCode=subprocess.check_output([bun,'--smol',str(t/'living-run-signature.ts'),'strict'],text=True,cwd=r).strip()
fleetCode=subprocess.check_output([sys.executable,str(t/'equipment-fleet.py'),'strict','--signature'],text=True,cwd=r).strip()
aliases=read(living/'code-aliases.json')
prior={}
for line in (fleets/'history.jsonl').read_text().splitlines():
    j=json.loads(line)
    if j['mode']=='strict' and j.get('completed') and j.get('passed') and j.get('current'):prior[j['asset'],j['input']]=j
rows=[];counts=Counter();recheck=[]
for p in sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json')):
    name=p.parent.name;category=p.parent.parent.name;h=sha256(p.read_bytes()).hexdigest();state='Pending'
    if category.endswith(('vegetation','animals')):
        j=read(living/(name+'-strict.json'));code=j.get('code')
        if j.get('hash')==h and 'scale' not in j and ('scales' not in j or sorted(map(int,j['scales']))==[50,100,200,500]):
            state='Findings' if j['findings'] else 'Pass' if aliases.get(code,code)==livingCode else 'Pass on earlier validator'
    else:
        j=read(fleets/(name+'-strict.json'));matched=j.get('input')==h and j.get('current') and j.get('completed')
        if matched and not j['passed']:state='Findings'
        else:
            last=j if matched else prior.get((name,h))
            if last:state='Pass' if last['code']==fleetCode else 'Pass on earlier validator'
    counts[state]+=1
    if state in ('Pending','Findings'):recheck.append(name+' ('+state.lower()+')')
    rows.append(f"| {category} | {name} | `{h[:16]}` | {state} |")
stamp=datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')
text=['# Per-model volume verification','',f"At {stamp}, {counts['Pass']+counts['Pass on earlier validator']} of {len(rows)} current model documents have a complete passing measurement at all four scales. Of those passes, {counts['Pass']} used the current validator. {counts['Findings']} documents have a recorded finding on the same source and {counts['Pending']} await a complete source-matched run.",'','Each check covers body, thermal and structural geometry. Whole-model checks include physical bounds, silhouette agreement, features, cross-scale geometry and the binding of thermal surfaces and structural members. The architecture and living-model validators are tracked separately. The table includes complete checks only. Separate evidence covers new model parts while the complete checks run.','',
'| Family | Asset | Source SHA-256 prefix | Whole-model check |','| --- | --- | --- | --- |',*rows,'']
(t/'digital-volume-status.md').write_text('\n'.join(text))
print(text[2])
