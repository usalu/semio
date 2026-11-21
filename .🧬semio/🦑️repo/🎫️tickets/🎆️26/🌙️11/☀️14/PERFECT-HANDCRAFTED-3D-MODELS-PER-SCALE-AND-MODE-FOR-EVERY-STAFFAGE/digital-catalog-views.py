"""🖼 Render the four body, temperature and structural panels from each current catalogue document."""
from concurrent.futures import ThreadPoolExecutor
from hashlib import sha256
from pathlib import Path
import os,json,subprocess,sys
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';out=g/'living-current-views';out.mkdir(exist_ok=True)
bun=os.environ['BUN_EXE']
assets=list(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'))
def order(p):
 group=p.parent.parent.name
 return (0 if group.endswith('animals') else 1 if group.endswith('humans') else 2,p.stat().st_size)
def render(p):
 raw=p.read_bytes();h=sha256(raw).hexdigest();a=json.loads(raw)['asset'];name=a['id'];pose=a['presentation']['defaultPose']
 dest=out/f'{name}-{pose}-all-temperature-{h[:10]}.png'
 del a,raw
 if dest.is_file():return
 log=g/('digital-paint-'+name+'.log')
 with log.open('w') as stream:
  done=subprocess.run([sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/'living-visual.ts'),name,'all',pose,str(p),'--temperature'],stdout=stream,stderr=stream,env={**os.environ,'EQUIPMENT_RSS_MIB':'550','EQUIPMENT_HEADROOM_PERCENT':'48'})
 print(name,'rendered' if done.returncode==0 and dest.exists() and h==sha256(p.read_bytes()).hexdigest() else 'interrupted',flush=True)
with ThreadPoolExecutor(max_workers=2)as pool:list(pool.map(render,sorted(assets,key=order)))
