"""🪜 Keep the folded tread, deck, grab tube and foot stock at their physical sections at architectural scales."""
import math
import json
from pathlib import Path
from runpy import run_path

HERE=Path(__file__).resolve().parent
build=run_path(str(HERE/'r5-stepladder-build.py'))
k,draw,volume=[build['module'](name) for name in ['kit','draw','volumes']]
F,R=k.FRONT,k.REAR
NOTE={'en':'Four 80 × 25 mm treads, the 26 mm platform rim and the 28 mm round grab tube retain their physical sections. The rubber front and rear shoe soles are 38 and 32 mm wide. The paper outline carries the thin sections at this scale.', 'de':'Vier 80 × 25 mm große Trittstufen, der 26-mm-Plattformrand und das 28-mm-Griffrohr behalten ihre Querschnitte. Vordere und hintere Gummisohlen sind 38 und 32 mm breit. Im Druck macht die Kontur die dünnen Querschnitte lesbar.'}

def quad(x0,y0,x1,y1):
    return [[x0,y0],[x1,y0],[x1,y1],[x0,y1]]

def author(a):
    if 'representations' in a:
        for shapes in a['representations']['100']['geometry'].values():
            for shape in shapes:
                if shape['id'].startswith('foot-rear-') and shape.get('filled'):shape['penMm']=.2
    for d in (200,500):
        v=a['volumes'][str(d)]; body={s['id']:s for s in v['body']}; copies={};views={name:{} for name in ['elevation','front','plan']}
        def drawpart(name,bone,view,points,binding,z,tone,pen=.13):
            pen=.20 if d==500 or name.startswith("foot-") else pen
            views[view][name]=draw.fill(name,bone,'body',z,tone,draw.E(bone,points) if view=='elevation' else points,binding,pen)
        for i,y in enumerate(k.TREAD_Y):
            sid='tread-'+str(i+1); thick=k.TREAD[1];x1=k.fx(y-thick/2)+k.TREAD_FRONT;x0=x1-k.TREAD[0];h=k.tread_half(y)
            section=quad(x0,y-thick,x1,y)
            if d==200:section=[[x0,y-thick],[x1-.006,y-thick],[x1,y-thick+.006],[x1,y],[x0,y]]
            copies[sid]=k.slab(sid,F,volume.AL,'body',.6,section,-h,h)
            drawpart(sid,F,'elevation',section,draw.TR,2.5,.6)
            drawpart(sid,F,'front',quad(-h,y-thick,h,y),draw.TR,2.5,.6)
            drawpart(sid,F,'plan',quad(x0,-h,x1,h),draw.TR,2.5,.6)
        y=k.H_PLATFORM;x0,x1=k.DECK_X0,k.DECK_X1
        section=k.deck_profile(False) if d==200 else [[x0,y-.026],[x0+.004,y-.026],[x0+.004,y-.004],[x1-.004,y-.004],[x1-.004,y-.026],[x1,y-.026],[x1,y],[x0,y]]
        copies['platform']=k.slab('platform',F,volume.AL,'body',.6,section,-k.PLATFORM_HALF,k.PLATFORM_HALF)
        drawpart('platform',F,'elevation',quad(x0,y-.026,x1,y),draw.PL,2.4,.6)
        drawpart('platform',F,'front',quad(-k.PLATFORM_HALF,y-.026,k.PLATFORM_HALF,y),draw.PL,3.3,.6)
        y=k.BAR_Y;r=k.BAR_R;half=k.fz(y)-k.STILE[1]/2+.003;n=12 if d==200 else 8
        outline=[[r*math.cos(2*math.pi*j/n),r*math.sin(2*math.pi*j/n)] for j in range(n)]
        copies['grab-bar']=k.bar('grab-bar',F,volume.AL,'body',.5,outline,[k.fx(y),y,-half],[k.fx(y),y,half],[1,0,0]);drawpart('grab-bar',F,'front',quad(-half,y-r,half,y+r),draw.FR,3.1,.5)
        for end,bone in [('front',F),('rear',R)]:
            front=end=='front';zc=k.fz(0) if front else k.rz(0);h,w,_=k.SHOE_F if front else k.SHOE_R
            detailed=k.shoe_profile(front);section=detailed if d==200 else [detailed[j] for j in (0,1,4,5)]
            drawpart('foot-'+end,bone,'elevation',section,draw.FT,3.4 if front else 2.95,.12)
            for side,sg in k.SIDES:
                sid='shoe-'+end+'-'+side;zc1=sg*zc
                copies[sid]=k.slab(sid,bone,volume.RB,'body',.12,section,zc1-w,zc1+w)
                drawpart('foot-'+end+'-'+side,bone,'front',quad(zc1-w,0,zc1+w,h),draw.FT,3.4 if front else 2.2,.12)
                drawpart('foot-'+end+'-'+side,bone,'plan',quad(min(q[0] for q in section),zc1-w,max(q[0] for q in section),zc1+w),draw.FT,1.1,.12)
        v['body']=[copies.get(s['id'],s) for s in v['body']];pads={s['id']:s for s in volume.supports(.006 if d==200 else .008)};v['structure']=[pads.get(s['id'],s) for s in v['structure']];v['decisions'][0]=NOTE
        if 'representations' in a:
            rep=a['representations'][str(d)]
            for name,changes in views.items():
                shapes=rep['geometry'][name];seen={s['id'] for s in shapes}
                rep['geometry'][name]=[changes.get(s['id'],s) for s in shapes]+[s for id,s in changes.items() if id not in seen]
            rep['decisions']['thinElements']=NOTE
            if d==500:
                for name in ['elevation','front']:
                    rep['geometry'][name]=[s for s in rep['geometry'][name] if s['id'] not in ('tread-2','tread-4')]
                right=k.fx(k.TREAD_Y[0]-k.TREAD[1]/2)+k.TREAD_FRONT
                left=k.fx(k.TREAD_Y[-1]-k.TREAD[1]/2)+k.TREAD_FRONT-k.TREAD[0]
                h0,h1=k.tread_half(k.TREAD_Y[-1]),k.tread_half(k.TREAD_Y[0])
                bank=draw.fill('tread-bank',F,'body',2.5,.6,[[left,-h0],[right,-h1],[right,h1],[left,h0]],draw.TR,.20)
                rep['geometry']['plan']=[s for s in rep['geometry']['plan'] if not s['id'].startswith('tread-')]+[bank]
                rep['decisions']['merged']={'en':'Alternating treads give two separated marks across the full print band. All four aluminium treads remain in the digital model.','de':'Zwei sichtbare Wechselstufen bleiben im ganzen Druckbereich getrennt. Das digitale Modell enthält alle vier Aluminiumstufen.'}

def apply():
    lib=run_path(str(HERE/'r5-range-gaps-lib.py'))
    catalog=next(HERE.parents[6].glob('🎓*/*/*staffage/🗂*/*/stepladder/🔣️.json'))
    for path in [catalog,HERE/'volumes/objects/stepladder.json']:
        before=path.read_text();doc=json.loads(before);asset=doc.get('asset',doc);author(asset);updated=before
        for key in ['representations','volumes'] if path==catalog else ['volumes']:
            for d in (['100','200','500'] if key=='representations' else ['200','500']):
                start,end=lib['key_value'](updated,d,*lib['key_value'](updated,key,0,len(updated)));line=updated[updated.rfind('\n',0,start)+1:start];indent=len(line)-len(line.lstrip());updated=updated[:start]+lib['render'](asset[key][d],indent,2)+updated[end:]
        assert json.loads(updated)==doc
        if path.read_text()!=before:raise RuntimeError(path)
        if updated!=before:path.write_text(updated)

if __name__=='__main__':apply()
