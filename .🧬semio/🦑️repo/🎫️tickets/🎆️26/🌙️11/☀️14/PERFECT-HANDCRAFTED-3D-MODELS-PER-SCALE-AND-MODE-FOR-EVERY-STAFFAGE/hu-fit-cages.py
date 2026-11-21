"""🫁 Sets each authored rib cage into the figure's live structural model and matching author inputs."""
import json
import os
from pathlib import Path
import re
from runpy import run_path
import subprocess

ticket=Path(__file__).resolve().parent
splice=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
labels={
    'ten rib hoops':'twelve pairs of descending ribs with a costal margin and floating ends',
    'zehn Rippenbögen':'zwölf absteigende Rippenpaare mit Rippenbogen und freien Enden',
    'five rib hoops':'six bilateral rib courses including a floating pair',
    'fünf Rippenbögen':'sechs beidseitige Rippenzüge mit einem freien Paar',
    'three rib hoops':'three bilateral rib courses with the lower costal arch',
    'drei Rippenbögen':'drei beidseitige Rippenzüge mit dem unteren Rippenbogen',
    'rib-cage mass':'two open paired rib courses and a sternal bar',
    'Brustkorbmasse':'zwei offene Rippenpaare und ein Brustbeinstab',
}
def labelled(text):
    for a,b in labels.items():
        text=text.replace(a,b)
    return text
for slug,author in [('tour-guide','r3-tour-guide'),('person-with-umbrella','r5-person-with-umbrella')]:
    bun=os.environ.get('BUN_EXE','bun')
    src="import {ribCage} from './hu-thorax.ts'; console.log(JSON.stringify(Object.fromEntries([50,100,200,500].map(n=>[n,ribCage("+json.dumps(slug)+",n)]))))"
    cages=json.loads(subprocess.check_output([bun,'-e',src],cwd=ticket))
    for suffix in ['s50','coarse']:
        p=ticket/(author+'-'+suffix+'.ts')
        old=p.read_text()
        if 'import { ribCage }' in old:continue
        code=old.replace('import type { Json }','import { ribCage } from "./hu-thorax.ts";\nimport type { Json }',1)
        if code==old:
            a=old.index('import ')
            code=old[:a]+'import { ribCage } from "./hu-thorax.ts";\n'+old[a:]
        if suffix=='s50':
            i=code.index('  const RIB_TILT' if '  const RIB_TILT' in code else '  const hoops:')
            j=code.index('  m(loft({ id: "sternum"',i)
            code=code[:i]+'  out.push(...ribCage("'+slug+'", 50));\n'+code[j:]
        else:
            code,n=re.subn(r'    m\(ellipsoid\(\{ id: "rib-cage".*?\n','    out.push(...ribCage("'+slug+'", scale));\n',code)
            assert n==1
            i=code.index('    const hoops:');j=code.index('    m(loft({ id: "sternum"',i)
            code=code[:i]+'    out.push(...ribCage("'+slug+'", scale));\n'+code[j:]
        assert p.read_text()==old
        p.write_text(labelled(code))
    p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=p.read_text();code=old
    for k,cage in cages.items():
        data=json.loads(code)['asset']['volumes'][k]
        if any(q['id']=='costal-2-l' or q['id']=='costal-1-l' for q in data['structure']):continue
        a=code.index('"'+k+'": {',code.index('"volumes": {'))
        b=code.index('"structure": [',a)
        pattern=r'\{\n            "id": "rib-cage(?:-\d+)?"'
        first=re.search(pattern,code[b:])
        i=b+first.start()
        n=sum(q['id'].startswith('rib-cage') for q in data['structure'])
        end=i
        for _ in range(n):
            located=re.search(pattern,code[end:]);end+=located.start()
            end=splice['member_end'](code,end)
        serial=',\n          '.join(splice['dump'](solid,5,'compact') for solid in cage)
        code=code[:i]+serial+code[end:]
    code=labelled(code)
    volumes=json.loads(code)['asset']['volumes']
    for k,v in volumes.items():
        ribs=[q for q in v['structure'] if q['id'].startswith('costal-')]
        assert len(ribs)=={ '50':24,'100':12,'200':6,'500':4}[k]
        assert all(q['id']!='rib-cage' for q in v['structure'])
        assert len(set(q['id'] for q in v['structure']))==len(v['structure'])
    assert p.read_text()==old
    if code!=old:p.write_text(code)
    snap=ticket/'volumes'/'humans'/(slug+'.json')
    stored=snap.read_text();doc=json.loads(stored)
    for k,vol in volumes.items():
        doc['volumes'][k]['structure']=vol['structure']
        doc['volumes'][k]['decisions']=vol['decisions']
    updated=json.dumps(doc,ensure_ascii=False,separators=(',',':'))+'\n'
    assert snap.read_text()==stored
    if updated!=stored:snap.write_text(updated)
