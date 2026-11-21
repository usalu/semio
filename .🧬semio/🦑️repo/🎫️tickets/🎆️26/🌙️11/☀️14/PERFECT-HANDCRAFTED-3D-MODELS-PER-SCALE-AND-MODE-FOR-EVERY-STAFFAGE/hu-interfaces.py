"""🧶 Checks and authors named thermal interfaces for fitted headwear, spectacles and undergarments."""
import json,re,sys
from pathlib import Path
from runpy import run_path

ticket=Path(__file__).resolve().parent
root=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
dump=sp['dump'];end=sp['member_end']
check='--check' in sys.argv
caps={'parent-carrying-toddler':{'beanie','toddler-hat'},'skateboarder':{'beanie'},'leaning-person':{'hair'},'tour-guide':{'hair'}}
repairs={'person-cooking','pregnant-person','sleeping-person',*caps}
air={'id':'still-air','title':{'en':'Still air','de':'Ruhende Luft'},'emissivity':.9,'source':'authored-estimate'}
hair={'id':'hair','title':{'en':'Dark hair under the beanie','de':'Dunkles Haar unter der Mütze'},'emissivity':.95,'solarAbsorptivity':.85,'source':'authored-estimate'}
for p in sorted(root.glob('*/*')):
    original=p.read_text();text=original
    a=json.loads(text)['asset'];slug=a['id']
    fixes=[]
    def replace_id(section,id,items):
        global text
        i=text.index('"id": "'+id+'"',section);i=text.rindex('{',0,i)
        j=end(text,i);depth=(i-text.rindex('\n',0,i)-1)//2
        if not items:
            k=j
            while text[k].isspace():k+=1
            if text[k]==',':
                k+=1
                while text[k].isspace():k+=1
                text=text[:i]+text[k:]
            else:
                k=i-1
                while text[k].isspace():k-=1
                assert text[k]==','
                text=text[:k]+text[j:]
            return
        text=text[:i]+(',\n'+'  '*depth).join(dump(q,depth,'compact') for q in items)+text[j:]
    def add_mat(mat):
        if any(m['id']==mat['id'] for m in a['materials']):return
        marker=text.index('"materials": [');m=a['materials'][-1]
        replace_id(marker,m['id'],[m,mat]);a['materials'].append(mat);fixes.append(mat['id'])
    def liststart(d,label):return text.index('"'+label+'": [',text.index('"'+d+'": {',text.index('"volumes": {')))
    def regstart():return text.index('"regions": [',text.index('"thermal": {\n      "environment"'))
    gap={}
    for r in a['thermal']['regions']:
        changed=False
        if r['id'] in caps.get(slug,set()) and r['kind']!='clothing':
            r['kind']='clothing';changed=True
        for l in r['layers']:
            if l['kind']=='air-gap' and l['material'] not in ('still-air','air','harness-air'):
                gap[r['id'],l['id']]=l['material'];l['material']='still-air';changed=True
        if changed:
            replace_id(regstart(),r['id'],[r]);fixes.append(r['id'])
    if gap:add_mat(air)
    if slug=='parent-carrying-toddler':add_mat(hair)
    for d,v in a['volumes'].items():
        by={q['id']:q for q in v['body']}
        if slug=='parent-carrying-toddler' and 'hair' in by and by['hair']['material']!='hair':
            q=by['hair'];q['material']='hair';replace_id(liststart(d,'body'),q['id'],[q]);fixes.append(q['id'])
        for q in list(v['thermal']):
            change=False
            oldMat=gap.get((q['region'],q.get('layer')))
            if oldMat is not None and q.get('material')==oldMat:
                q['material']='still-air';change=True
            if slug=='parent-carrying-toddler' and q.get('of')=='hair' and q.get('layer')=='beanie':
                q['layer']='beanie-air';change=True
            if slug=='leaning-person' and q['id']=='th-hair-under-beanie' and q['material']!='hair':
                q['material']='hair';change=True
            if slug in {'person-cooking','pregnant-person'} and q.get('of') in ['brow-l','brow-r'] and q['region']=='face-skin':
                q.update(region='hair',layer='hair');change=True
            if change:
                replace_id(liststart(d,'thermal'),q['id'],[q]);fixes.append(q['id'])
        if slug=='sleeping-person' and d=='50' and 'shorts-leg-l' not in by:
            for side,length in [('l',.119),('r',.113)]:
                skin=by['thigh-'+side];first=skin['sections'][0]
                rings=[(-.018,.091,.086),(.034,.096,.092),(.088,.097,.093),(length-.006,.094,.091),(length,.095,.092)]
                shorts={**skin,'id':'shorts-leg-'+side,'material':'cotton-underwear','tone':by['shorts']['tone'],'sections':[{**first,'at':[x,.001 if x>0 else 0,0],'rx':rx,'ry':ry} for x,rx,ry in rings]}
                replace_id(liststart(d,'body'),skin['id'],[skin,shorts])
                for limb in ['thigh','shank']:
                    key='th-'+limb+'-'+side
                    ref=next(x for x in v['thermal'] if x['id']==key)
                    replace_id(liststart(d,'thermal'),key,[{**ref,'id':'th-shorts-leg-'+side,'of':shorts['id']}] if limb=='thigh' else [])
            fixes.append('two pyjama shorts cuffs')
    if fixes:
        if check:raise ValueError((slug,'pending',len(fixes)))
        assert json.loads(text) and p.read_text()==original
        p.write_text(text)
        print(slug,len(fixes))
