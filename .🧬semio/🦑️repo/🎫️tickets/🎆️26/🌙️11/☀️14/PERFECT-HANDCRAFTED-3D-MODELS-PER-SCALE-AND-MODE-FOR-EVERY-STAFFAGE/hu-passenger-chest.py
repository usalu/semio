"""🧒 Models the two passengers' short, relatively transverse ribs in their actual carrying frames."""
import json,re
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent
h=run_path(str(t/'hu-open-ribs.py'));sp=h['sp']
charts={
 'parent-with-stroller':[(213,34,201),(205,44,191),(196,50,180),(186,55,169),(176,59,157),(166,62,144),(155,63,132),(143,62,120),(131,60,108),(119,56,98),(107,47,92),(95,35,86)],
 'parent-carrying-toddler':[(235,41,222),(225,53,211),(214,62,199),(202,70,185),(190,75,171),(177,79,157),(164,81,143),(151,80,129),(138,77,116),(125,73,104),(112,62,97),(99,45,90)]}
for slug,chart in charts.items():
    seated=slug=='parent-with-stroller';member='child' if seated else 'toddler-torso'
    prefix='child' if seated else 'toddler'
    ref=[[105,-30,0],[144,-23,0],[179,-16,0],[201,-12,0]] if seated else [[141,-84,0],[177,-80,0],[201,-73,0],[222,-61,0]]
    p=next((h['hum']/slug).glob('*'));old=p.read_text();text=old;asset=json.loads(old)['asset']
    for k,v in asset['volumes'].items():
        sc=int(k);spine=next(q for q in v['structure'] if q['id']==prefix+'-spine')
        if any(q['id']==prefix+'-costal-7-l' for q in v['structure']):continue
        axis=[q['at'] for q in spine['sections']]
        vert=sorted([(y*1000,-x*1000,z*1000) if seated else (x*1000,y*1000,z*1000) for x,y,z in axis])
        courses=h['cage'](chart,ref,vert,sc)
        if sc==50:
            courses.append(h['tube']('sternum',[(x,y,0) for x,y,_ in ref],3.8))
        for q in courses:
            q.update(id=prefix+'-'+q['id'],bone=member,member=member,tone=.93)
            width=(1.25 if q['id'].endswith('sternum') else 1.95) if sc>=200 else (3.8 if sc==50 else 7.5)
            for section in q['sections']:
                x,y,z=section['at']
                if seated:section['at']=[round(-y,5),x,round(axis[0][2]+z,5)]
                section['rx']=section['ry']=width/1000
        idx=text.index('"'+k+'": {',text.index('"volumes": {'));i,j=sp['member_range'](text[idx:],'        ','structure');i+=idx;j+=idx
        source=text[i:j];chunks=re.split(r',\n(?=          \{)',source);updated=[]
        for q,chunk in zip(v['structure'],chunks):
            if q.get('bone')!=member or ('rib' not in q['id'] and 'sternum' not in q['id']):updated.append(chunk)
            if q is spine:updated.extend('          '+sp['dump'](r,5,'compact') for r in courses)
        assert len(chunks)==len(v['structure'])
        text=text[:i]+',\n'.join(updated)+text[j:]
        note=v['decisions'][-1];n=len(h['levels'][sc])
        for lang,extra in [('en',f' The passenger has {n} paired rib courses on their own spine.'),('de',f' Beim Kind sitzen {n} beidseitige Rippenzüge an seiner eigenen Wirbelsäule.')]:
            before=note[lang]
            after=re.sub(r'four rib hoops|vier Rippen(?:bögen|ringe|reifen)', 'twelve pairs of ribs' if lang=='en' else 'zwölf Rippenpaare',before)
            text=text.replace(json.dumps(before,ensure_ascii=False),json.dumps(after+extra,ensure_ascii=False),1)
    if text==old:continue
    doc=json.loads(text)['asset']
    assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/(slug+'.json');orig=snap.read_text();saved=json.loads(orig)
    for k,v in doc['volumes'].items():
        saved['volumes'][k]['structure']=v['structure'];saved['volumes'][k]['decisions']=v['decisions']
    assert snap.read_text()==orig;snap.write_text(json.dumps(saved,ensure_ascii=False,separators=(',',':'))+'\n')
    print(slug)
