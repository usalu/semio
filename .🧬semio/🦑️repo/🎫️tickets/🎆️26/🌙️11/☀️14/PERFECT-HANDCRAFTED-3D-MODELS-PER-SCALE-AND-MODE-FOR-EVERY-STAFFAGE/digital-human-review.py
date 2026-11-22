"""🧍 Render the human catalogue’s four scales in two named poses with bounded shared-memory grants."""
from hashlib import sha256
from pathlib import Path
import json,os,shutil,subprocess,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated';views=g/'living-current-views';root=t.parents[6]
bun=os.environ.get('BUN_EXE') or shutil.which('bun') or next(p for p in root.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
gestures=json.loads((t/'digital-human-gestures.json').read_bytes())
requested=sys.argv[1:] or list(gestures)
for name in requested:
 cat=next(root.glob('🎓*/*architecture/*staffage/*catalog/*humans/'+name+'/*.json'))
 raw=cat.read_bytes();h=sha256(raw).hexdigest();a=json.loads(raw)['asset'];data=[]
 for i,pose in enumerate([a['presentation']['defaultPose'],gestures[name]]):
  options=(['--opposite'] if i else [])+['--presentation','--shaded','--temperature']
  path=views/('-'.join([name,pose,'all',*map(lambda s:s[2:],options),h[:10]])+'.png')
  if len(str(path).encode('utf-16-le'))//2>248:
   path=views/('-'.join([name,pose,'all',''.join(s[2] for s in options),h[:10]])+'.png');options.append('--short-name')
  if not path.is_file():
   cmd=[str(bun),'--smol',str(t/'living-visual.ts'),name,'all',pose,str(cat),*options]
   for mib in (320,480,690):
    result=subprocess.run([sys.executable,str(t/'equipment-guard.py'),*cmd],capture_output=True,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':str(mib),'EQUIPMENT_HEADROOM_PERCENT':'40'})
    if result.returncode==0:break
    if result.returncode!=75:raise RuntimeError(result.stderr)
   if not path.is_file():raise RuntimeError(name+' '+pose+' rendering interrupted')
  data.append({'pose':pose,'path':str(path),'opposite':bool(i)})
 if sha256(cat.read_bytes()).hexdigest()!=h:raise RuntimeError(name+' changed during rendering')
 (g/('human-review-'+name+'.json')).write_text(json.dumps({'name':name,'hash':h,'views':data}))
 print(name,h[:12],flush=True)
