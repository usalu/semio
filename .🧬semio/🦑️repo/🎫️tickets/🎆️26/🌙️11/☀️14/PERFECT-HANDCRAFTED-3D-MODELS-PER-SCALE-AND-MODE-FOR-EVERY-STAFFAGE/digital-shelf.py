"""📚 Author real board sections, pin supports and independently deflecting shelf halves at every architectural reduction."""
from pathlib import Path
from runpy import run_path
import json
import math

T=Path(__file__).resolve().parent
model=run_path(str(T/'r3-bookshelf-model.py'));K=model['K'];F='frame'
NOTE={'en':'The 3 mm hardboard back fits behind five 280 mm deep shelves. The carcass, plinth and shelves retain their 18 mm board stock. Each shelf bears on four 5 mm pins, with two bending halves spanning between the supports.', 'de':'Hinter fünf 280 mm tiefen Fachböden sitzt eine 3-mm-Hartfaserrückwand. Korpus, Sockelblende und Fachböden behalten ihre Plattenstärke von 18 mm. Jeder Fachboden liegt auf vier 5-mm-Stiften. Seine zwei biegsamen Hälften spannen zwischen den Auflagern.'}

def rect(sid,bone,x,y,w,h,material,region,z=5,tone=.7):
    s=dict(id=sid,bone=bone,role='body',z=z,filled=True,tone=tone,kind='rect',x=x,y=y,w=w,h=h,material=material,penMm=.2)
    if region:s['region']=region
    return s

def pins(n,d):
    items=model['pins'](n)
    for s in items:
        if d==100:
            s['profile']=[[-.008,0],[-.008,.0025],[.009,.0025],[.009,0]]
            continue
        sign=s['axis'][0];ox,oy,oz=s['origin'];sides=8 if d==200 else 4
        s.update(kind='extrude',plane='yz',profile=[[round(oy+.0025*math.cos(2*math.pi*i/sides),7),round(oz+.0025*math.sin(2*math.pi*i/sides),7)] for i in range(sides)],**{'from':round(min(ox-.008*sign,ox+.009*sign),5),'to':round(max(ox-.008*sign,ox+.009*sign),5)})
        s.pop('origin');s.pop('axis')
    return items

def shelves(n,d):
    items=model['shelf_halves'](n)
    if d==500:return items
    for part in items:
        sid=part['id'];sx,sy,sz=part['size'];x,y,z=part.pop('transform')['translation'];part.pop('size')
        sign=-1 if sid.endswith('-r') else 1
        z0,z1=round(z-sz/2,5),round(z+sz/2,5)
        outline=[[0,z0+.001],[sign*.001,z0],[sign*.017,z0],[sign*.018,z0+.001],[sign*.018,z1],[0,z1]]
        if d<=100:
            radius=.001
            n=4 if d==50 else 2
            outline=[[round(sign*(radius+radius*math.sin(-math.pi/2-j*math.pi/(2*n))),7),round(z0+radius+radius*math.cos(-math.pi/2-j*math.pi/(2*n)),7)] for j in range(n+1)]
            outline.extend([[round(sign*(.018-radius+radius*math.sin(math.pi-j*math.pi/(2*n))),7),round(z0+radius+radius*math.cos(math.pi-j*math.pi/(2*n)),7)] for j in range(n+1)])
            outline.extend([[sign*.018,z1],[0,z1]])
        part.update(kind='extrude',plane='yz',profile=outline,**{'from':round(x-sx/2,5),'to':round(x+sx/2,5)})
    return items

def author(a):
    finest=a['volumes']['50'];rounded={s['id']:s for n in range(1,6) for s in shelves(n,50)}
    finest['body']=[rounded.get(s['id'],s) for s in finest['body']]
    for d in (100,200,500):
        v=a['volumes'][str(d)];existing={s['id']:s for s in v['body']};physical=[];shelf_parts=[];pin_parts=[]
        if d>100:
            physical=[model['carcass'](.018,.060),K.box('back',F,K.HARDBOARD,(.018,.782),(.078,1.782),(.1375,.1405)),K.box('plinth',F,K.BOARD,(.018,.782),(0,.060),(-.120,-.102),tone=.4)]
        else:physical=[s for s in v['body'] if not s['id'].startswith(('shelf-','pin-'))]
        for n in range(1,6):
            shelf_parts.extend(shelves(n,d));pin_parts.extend(pins(n,d))
        v['body']=physical+shelf_parts+pin_parts
        v['thermal']=model['thermal'](str(d),v['body'])
        rest=[s for s in v['structure'] if not s['id'].startswith(('m-shelf-','m-pin-','bearing-','st-pin-')) and s.get('of') not in {p['id'] for p in pin_parts}]
        for s in rest:
            if s.get('part')=='support':
                x=.009 if 'left' in s['id'] else .791
                s.update(size=[.018,.004,.30],transform=dict(translation=[x,.002,0]))
        v['structure']=rest+[model['of'](s['id'],s['bone']) for s in shelf_parts+pin_parts]
        v['decisions'][0]=NOTE
        if 'representations' not in a:continue
        rep=a['representations'][str(d)];g=rep['geometry'];elev=[s for s in g['elevation'] if not s['id'].startswith(('shelf-','pin-')) and s['id']!='plinth'];front=[s for s in g['front'] if s['id']!='back' and not s['id'].startswith('shelf-')];front.append(rect('back',F,.1375,.078,.003,1.704,K.HARDBOARD,'back-panel',2,.9));elev.append(rect('plinth',F,.018,0,.764,.06,K.BOARD,'plinth',4,.4))
        for n,y in enumerate(K.SHELF_Y,1):
            for side in ('l','r'):
                b=f'shelf-{n}-{side}';front_half=side=='l'
                elev.append(rect(b,b,.001,-.018 if not front_half else 0,.381,.018,K.BOARD,'shelves'))
                elev.append(rect(f'pin-{n}-{side}',f'shelf-{n}',0 if front_half else .755,-.005,.009,.005,K.STEEL,None,5.5,.3))
            front.append(rect(f'shelf-{n}',f'shelf-{n}-l',-.145,y,.28,.018,K.BOARD,'shelves'))
        g['elevation']=elev;g['front']=front;rep['decisions']['thinElements']=NOTE

def apply():
    lib=run_path(str(T/'r5-range-gaps-lib.py'));catalog=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/bookshelf/🔣️.json'))
    for path in [catalog,T/'volumes/furniture/bookshelf.json']:
        original=path.read_text();doc=json.loads(original);a=doc.get('asset',doc);author(a);s=original
        for key in ['volumes','representations'] if path==catalog else ['volumes']:
            for d in (['50','100','200','500'] if key=='volumes' else ['100','200','500']):
                lo,hi=lib['key_value'](s,d,*lib['key_value'](s,key,0,len(s)));line=s[s.rfind('\n',0,lo)+1:lo];s=s[:lo]+lib['render'](a[key][d],len(line)-len(line.lstrip()),2)+s[hi:]
        assert json.loads(s)==doc
        if path.read_text()!=original:raise RuntimeError(path)
        if s!=original:path.write_text(s)

if __name__=='__main__':apply()
