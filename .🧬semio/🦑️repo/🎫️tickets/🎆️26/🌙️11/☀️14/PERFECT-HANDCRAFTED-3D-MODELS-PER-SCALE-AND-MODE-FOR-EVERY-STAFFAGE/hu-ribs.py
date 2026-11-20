"""🦴 Fits the individual costal arches of the cyclist, schoolchild and skater to their own thoracic vertebrae. Stations are millimetres."""
from pathlib import Path
import json
import re
from runpy import run_path

ticket=Path(__file__).resolve().parent
splice=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
rows={
 'cyclist':[
 (404,11,53,382,-83),(389,13,75,359,-108),(371,15,93,336,-127),(353,17,107,313,-139),
 (335,18,118,292,-149),(317,16,124,271,-153),(298,13,128,248,-153),(278,10,130,221,-150),
 (257,6,128,198,-143),(235,1,125,174,-133),(212,-7,112,155,-111),(190,-14,89,142,-81)],
 'child-on-kick-scooter':[
 (311,33,45,302,-29),(297,41,61,283,-39),(281,46,75,262,-49),(266,47,85,241,-57),
 (249,49,92,221,-62),(233,49,97,201,-66),(215,50,98,182,-66),(197,48,97,163,-64),
 (177,45,93,143,-63),(157,39,88,126,-58),(138,35,77,112,-48),(119,32,60,100,-31)],
 'skateboarder':[
 (428,14,53,410,-65),(409,18,75,385,-86),(390,20,94,362,-100),(370,21,111,337,-113),
 (348,22,124,313,-120),(327,21,130,291,-124),(305,18,134,265,-126),(283,16,134,236,-124),
 (260,12,131,210,-120),(237,6,125,186,-115),(213,1,106,163,-98),(188,-4,84,147,-68)]}
strut={'cyclist':5.4,'child-on-kick-scooter':4,'skateboarder':5.7}
def sections(points,r):
    return [dict(at=[round(v*.001,5) for v in point],rx=round(r*.001,5),ry=round(r*.001,5)) for point in points]
def build(slug,scale,base):
    out=[]
    rr=strut[slug] if scale=='50' else 7.55
    def loft(id,points,r):
        p={k:v for k,v in base.items() if k not in ('id','sections')}
        p.update(id=id,sections=sections(points,r))
        return p
    chest=rows[slug]
    for i,(h,back,w,frontheight,front) in enumerate(chest,1):
        if scale=='100' and i%2==0:continue
        drop=h-frontheight
        depth=back-front
        start=[h,back,4]
        angle=[h-.08*drop,back+.11*depth,.31*w]
        flank=[h-.52*drop,back-.40*depth,w]
        ventral=[frontheight,front+.09*depth,.52*w]
        if i < 8:
            end=[frontheight,front,4]
        elif i <11:
            end=[chest[6][3]-(i-7)*4,chest[6][4]+(i-7)*3,(i-7)*10+2]
        elif i==11:
            end=[frontheight,back-.74*depth,.75*w]
        else:
            end=[frontheight,back-.48*depth,.91*w]
        pts=[start,angle,[h-.24*drop,back+.025*depth,.74*w],flank]
        if i<=10:
            pts.extend([[h-.81*drop,back-.76*depth,.88*w],ventral])
        if i<11 and scale=='50':pts.append([(end[0]+ventral[0])/2,(end[1]+ventral[1])/2-1,(end[2]+ventral[2])/2])
        pts.append(end)
        for side,sign in [('l',1),('r',-1)]:
            out.append(loft('costal-'+str(i)+'-'+side,[[x,y,z*sign] for x,y,z in pts],rr))
    sternum=[[h,f,0] for _,_,_,h,f in chest[:7]]
    out.append(loft('sternum',sternum if scale=='50' else sternum[::2],rr*1.6))
    return out
for slug in rows:
    path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=path.read_text()
    d=json.loads(old)['asset']
    text=old
    for scale in ('50','100'):
        vol=d['volumes'][scale]
        former=next(s for s in vol['structure'] if s['id']=='ribcage')
        pieces=build(slug,scale,former)
        i=text.index('{\n            "id": "ribcage"')
        j=splice['member_end'](text,i)
        fresh=',\n          '.join(splice['dump'](p,5,'compact') for p in pieces)
        text=text[:i]+fresh+text[j:]
        previous=len(vol['structure'])
        count=previous-1+len(pieces)
        for key,noun in [('en','solids'),('de','Körper')]:
            before=vol['decisions'][-1][key]
            after=before.replace(str(previous)+' '+noun,str(count)+' '+noun)
            after+=(' Twelve paired ribs follow the vertebrae to the sternum and costal margins. The two lowest pairs end freely.' if scale=='50' else ' Six bilateral rib courses mark the aperture, chest wall, costal arch and floating ribs.') if key=='en' else (' Zwölf Rippenpaare folgen den Wirbeln zum Brustbein und zu den Rippenbögen. Die zwei untersten Paare enden frei.' if scale=='50' else ' Sechs beidseitige Rippenzüge zeichnen die obere Öffnung, Brustwand, Rippenbögen und freien Rippen.')
            text=text.replace(json.dumps(before,ensure_ascii=False),json.dumps(after,ensure_ascii=False),1)
    assert path.read_text()==old
    json.loads(text)
    path.write_text(text)
