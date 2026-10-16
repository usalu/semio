import sys,re,json,collections; sys.path.insert(0,'.')
from ext7 import *
CAP=(3300,215000,26)
files=plugin_files()
rows=[]
for f in files:
    rel=f.replace(PLUGREL+'/','')
    if '/📦️packages/' in f or not f.endswith('.rs'): continue
    if not ('/🧪️tests/' in f or '/🧪️testing/' in f or rel.startswith('🧪️')): continue
    p=ROOT+f
    if not os.path.exists(p): continue
    t=open(p,encoding='utf-8').read(); m=mask(t)
    if re.search(r'\b(VcsArtifactApp|PluginBuilder|artifact_instance|app_testing|InstanceExtension)\b',m):
        rows.append(dict(file=rel,lines=t.count('\n'),bytes=len(t.encode())))
BIG={'🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs':5,'🧪️tests/🧪️time-travel/🦀️.rs':2}
items=[]
for r in rows:
    if r['file'] in BIG:
        n=BIG[r['file']]
        for k in range(n): items.append(dict(file=r['file'],part=k+1,of=n,lines=r['lines']//n+1,bytes=r['bytes']//n+1))
    elif r['file']=='🧪️testing/🦀️.rs': continue       # the law library: crate testing-laws
    elif r['file'].startswith('🏗️builder/'): continue    # builder crate tests stay in the builder crate (dir-hosted)
    elif r['file'].startswith('⚛️reactor/') or r['file'].startswith('🕹️interaction') or r['file'].startswith('🧩️extension') or r['file'].startswith('🖥️host') or r['file'].startswith('🧩️composition'): continue   # other packages' hosts
    else: items.append(dict(file=r['file'],part=1,of=1,lines=r['lines'],bytes=r['bytes']))
items.sort(key=lambda i:-i['lines'])
bins=[dict(lines=0,bytes=0,files=[]) for _ in range(12)]
over=[]
for it in items:
    for b in bins:
        if b['lines']+it['lines']<=CAP[0] and b['bytes']+it['bytes']<=CAP[1] and len(b['files'])<CAP[2]:
            b['lines']+=it['lines']; b['bytes']+=it['bytes']; b['files'].append(it); break
    else: over.append(it)
for k,b in enumerate(bins):
    print('T%d'%(10+k),b['lines'],b['bytes'],len(b['files']),[ (f['file'].split('/')[1][:24],f['part']) for f in b['files']][:8])
print('unplaced',[(o['file'],o['part']) for o in over])
json.dump(dict(bins=bins,unplaced=over),open(GEN+'/pwp7-tbins.json','w'),ensure_ascii=False,indent=1)
