"""🎽 Assigns the sweatband, name badge, eyebrows and hair to their independently modelled materials and surface fields."""
import json,re,math,sys
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
for slug in ['tour-guide','jogger','person-cooking','pregnant-person','parent-carrying-toddler']:
    p=next((root/slug).iterdir());old=p.read_text();text=old;a=json.loads(text)['asset']
    def replace(marker,id,items):
        global text
        j=text.index('"id": "'+id+'"',marker);i=text.rindex('{',0,j);j=sp['member_end'](text,i)
        depth=(i-text.rindex('\n',0,i)-1)//2
        text=text[:i]+(',\n'+'  '*depth).join(sp['dump'](q,depth,'compact') for q in items)+text[j:]
    def vol(d,field):return text.index('"'+field+'": [',text.index('"'+d+'": {',text.index('"volumes": {')))
    def regions():return text.index('"regions": [',text.index('"thermal": {\n      "environment"'))
    if slug in ['tour-guide','jogger']:
        jog=slug=='jogger';new='headband' if jog else 'badge'
        if new not in {r['id'] for r in a['thermal']['regions']}:
            if '--check' in sys.argv:raise ValueError(slug+' '+new)
            target='hair' if jog else 'torso-clothing'
            oldr=next(r for r in a['thermal']['regions'] if r['id']==target)
            if jog:
                r={**oldr,'id':new,'title':{'en':'Wicking nylon sweatband','de':'Schweißband aus feuchtigkeitsleitendem Nylon'},'kind':'clothing','material':'nylon-clothing','areaM2':.0154,'characteristicLengthM':.026,'geometry':'cylinder','layers':[{'id':'nylon-band','title':{'en':'Stretch-knit nylon','de':'Elastischer Nylonstrick'},'kind':'clothing','material':'nylon-clothing','thicknessM':.0013,'resistanceM2KW':.012,'vapourResistanceM2PaW':1.91,'heatCapacityJM2K':210,'source':'authored-estimate','reference':'iso-9920-2007'}],'body':{**oldr['body'],'skinFraction':.008,'metabolicShare':.002}}
                r.pop('areaBasis',None);oldr.update(areaM2=.0616)
                oldr['title']={'en':'Uncovered scalp hair','de':'Unbedecktes Kopfhaar'}
                oldr['body'].update(skinFraction=.032,metabolicShare=.008)
                oldr['layers'][0]['title']={'en':'Loose hair above and below the band','de':'Freies Haar ober- und unterhalb des Stirnbands'}
            else:
                r={'id':new,'title':{'en':'Pinned fabric name badge and clip','de':'Textiles Namensschild mit Metallclip'},'kind':'surface','model':'passive','material':'cotton-clothing','areaM2':.0052,'characteristicLengthM':.054,'geometry':'vertical-plate','skyViewFactor':.4,'layers':[{'id':'cloth-face','title':{'en':'Woven badge face','de':'Gewebte Schildfläche'},'kind':'clothing','material':'cotton-clothing','thicknessM':.0008,'resistanceM2KW':.003,'source':'authored-estimate'}],'heatCapacityJM2K':500,'source':'authored-estimate'}
            replace(regions(),target,[oldr,r])
            def bind(m):
                previous=json.loads(m[2])
                binding={'exposedFraction':1,'sunlitFraction':previous['sunlitFraction']}
                if jog:binding['wettedness']=.25
                else:binding['contact']={'fraction':.4,'tempC':26}
                return m[0]+',\n'+m[1]+'"'+new+'": '+sp['dump'](binding,len(m[1])//2,'compact')
            text,n=re.subn(r'(?m)^( +)"'+target+r'": (\{\n(?:(?!^\1\}).*\n)*?^\1\})',bind,text);assert n==len(a['poses'])
            for d,v in a['volumes'].items():
                by={q['id']:q for q in v['body']}
                mapped=False
                for q in v['thermal']:
                    if q.get('of') not in ([new] if jog else ['badge','badge-clip']):continue
                    q={**q,'region':new};q.pop('layer',None)
                    if jog:q['sectionFaces']=['-u','+u']
                    else:q['sectionFaces']=['+v','-u','+u','-w','+w'] if q['of']=='badge' else ['+v']
                    if d=='50' and (jog or q['of']=='badge'):q['layer']='nylon-band' if jog else 'cloth-face'
                    replace(vol(d,'thermal'),q['id'],[q]);mapped=True
                if not mapped:
                    last=v['thermal'][-1];base=next(q for q in a['volumes']['100' if jog else '50']['body'] if q['id']==new)
                    q={**base,'id':'th-'+new,'region':new}
                    if jog:
                        ax0,ax1,rx,ry=(.172,.199,.087,.104) if d=='200' else (.172,.20,.085,.101)
                        q['sections']=[{**base['sections'][0],'at':[ax0,0,0],'rx':rx,'ry':ry},{**base['sections'][1],'at':[ax1,0,0],'rx':rx-.002,'ry':ry-.002}]
                        q['sectionFaces']=['-u','+u']
                    else:
                        q['size']=[.086,.002,.054];q.pop('cornerRadius',None);q['sectionFaces']=['+v','-u','+u','-w','+w'];q['layer']='cloth-face'
                    replace(vol(d,'thermal'),last['id'],[last,q])
                if jog:
                    hair=by['hair'];q=next(k for k in v['thermal'] if k.get('of')=='hair')
                    band=by.get('headband');lo,hi=(band['sections'][0]['at'][0],band['sections'][-1]['at'][0]) if band else (ax0,ax1)
                    def ring(x):
                        for s0,s1 in zip(hair['sections'],hair['sections'][1:]):
                            if s0['at'][0]<=x<=s1['at'][0]:
                                u=(x-s0['at'][0])/(s1['at'][0]-s0['at'][0]);lerp=lambda v,w:round(v+u*(w-v),6)
                                return {**s0,'at':[lerp(v,w) for v,w in zip(s0['at'],s1['at'])],'rx':lerp(s0['rx'],s1['rx']),'ry':lerp(s0['ry'],s1['ry'])}
                        raise ValueError(x)
                    split=[]
                    for suffix,rings,faces in [('nape',[s for s in hair['sections'] if s['at'][0]<lo]+[ring(lo)],['+u']),('crown',[ring(hi)]+[s for s in hair['sections'] if s['at'][0]>hi],['-u'])]:
                        shape={**hair,'id':'th-hair-'+suffix,'region':'hair','sections':rings,'sectionFaces':faces}
                        if d=='50':shape['layer']='hair-pack'
                        split.append(shape)
                    replace(vol(d,'thermal'),q['id'],split)
    def shape(m):
        q=json.loads(m[0]);name=q['id']
        if slug=='jogger' and name=='headband':q['region']='headband'
        elif slug=='tour-guide' and name=='badge':q['region']='badge'
        elif slug=='parent-carrying-toddler' and name=='hair':q['material']='hair'
        elif slug in ('person-cooking','pregnant-person','jogger') and re.fullmatch('brow-[lr](-line)?',name):
            q['region']='hair';q['material']='hair'
        else:return m[0]
        return sp['dump'](q,6,'compact')
    text=re.sub(r'\{\n              "id": "(?:brow-[lr](?:-line)?|headband|badge|hair)".*?\n            \}',shape,text,flags=re.S)
    if '--check' in sys.argv:assert text==old,slug
    else:
        assert json.loads(text) and p.read_text()==old
        if text!=old:p.write_text(text)
    print(slug)
