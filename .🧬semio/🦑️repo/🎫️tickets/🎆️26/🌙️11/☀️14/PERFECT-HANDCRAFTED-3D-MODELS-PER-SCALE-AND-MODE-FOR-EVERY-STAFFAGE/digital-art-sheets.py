"""🖼 Index current four-scale art reviews and build category-specific contact sheets with numeric surface temperatures."""
from pathlib import Path
from hashlib import sha256
import os,json,sys,subprocess
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';out=g/'living-current-views';assets=sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'))
groups={};rows=[];done=0
for asset in assets:
 raw=asset.read_bytes();a=json.loads(raw)['asset'];category=a['category'];n=a['id'];h=sha256(raw).hexdigest();image=out/f"{n}-{a['presentation']['defaultPose']}-all-temperature-{h[:10]}.png"
 groups.setdefault(category,[]).append((n,h,image));done+=int(image.exists())
 rows.append(f"| {category} | {n} | `{h[:16]}` | "+('Rendered' if image.exists() else 'Pending')+' |')
for category,items in groups.items():
 for i in range(0,len(items),4):
  group=items[i:i+4];name=f'art-{category}-{1+i//4}'
  if not all(p.is_file() for _,_,p in group):continue
  dest=out/('contact-'+name+'.png');signature=''.join(h for _,h,_ in group);key=out/(name+'.sha')
  if dest.exists() and key.exists() and key.read_text()==signature:continue
  proc=subprocess.run([sys.executable,str(t/'equipment-guard.py'),os.environ['BUN_EXE'],'--smol',str(t/'living-contact.ts'),'--temperature',name,*[n for n,_,_ in group]],stdout=subprocess.DEVNULL,env={**os.environ,'EQUIPMENT_RSS_MIB':'200','EQUIPMENT_HEADROOM_PERCENT':'48'})
  if proc.returncode==0 and dest.exists():key.write_text(signature)
text=['# Four-scale visual review index','',f'{done} of {len(rows)} exact catalogue sources rendered in their presentation pose. Each source has four rows for the digital scales and three columns for the body, numeric surface temperature and structural anatomy. Source-matched render coverage is recorded below. Individual model reports record visual and dimensional findings.','','| Category | Asset | Source SHA-256 prefix | Render |','| --- | --- | --- | --- |',*rows]
(t/'digital-art-index.md').write_text('\n'.join(text)+'\n');print(text[2])
