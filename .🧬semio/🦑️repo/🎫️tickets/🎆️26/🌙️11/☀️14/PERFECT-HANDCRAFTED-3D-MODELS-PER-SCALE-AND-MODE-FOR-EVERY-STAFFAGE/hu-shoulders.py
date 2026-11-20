"""🧥 Models the shoulder girdle of the skater, accounting for the anteriorly placed arms at the ends of the shoulders."""
import json
from pathlib import Path
from runpy import run_path
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/skateboarder/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=path.read_text()
text=old
offset=text.index('"volumes": ')
for scale,v in json.loads(old)['asset']['volumes'].items():
    pieces=[]
    def add(id,rows,q=None,slender=False):
        sections=[dict(at=[x,y,z],rx=rw,ry=rd,**({'rotation':q} if q else {})) for x,y,z,rw,rd in rows]
        pieces.append(dict(id=id,kind='loft',bone='torso',material='skin',role='body',tone=.9,sections=sections,capStart=True,capEnd=True,member='torso',part='member',**({'slender':True} if slender else {})))
    if scale in ('50','100'):
        for side,g in [('l',1),('r',-1)]:
            clav=[[.41,-.066,.004,.011,.01],[.419,-.075,.04,.009,.008],[.43,-.079,.082,.008,.0075],[.44,-.067,.116,.0075,.008],[.444,-.045,.15,.009,.0085],[.4364,-.05,.178,.012,.01]]
            blade=[[.312,.056,.077,.011,.008],[.34,.058,.093,.023,.009],[.377,.054,.105,.035,.009],[.403,.043,.117,.043,.01],[.421,.025,.133,.033,.011],[.436,-.015,.154,.021,.012],[.437,-.046,.173,.013,.012]]
            for id,rows,q in [('clavicle-',clav,None),('scapula-',blade,[0,-.7071068,0,.7071068])]:
                if scale=='100':rows=[rows[i] for i in ([0,2,4,5] if id=='clavicle-' else [0,2,3,5,6])]
                add(id+side,[[x,y,z*g,rw,rd] for x,y,z,rw,rd in rows],q)
    else:
        span=[[.4364,-.05,-.178],[.439,-.05,-.144],[.43,-.078,-.084],[.41,-.065,0],[.43,-.078,.084],[.439,-.05,.144],[.4364,-.05,.178]]
        r=.0155 if scale=='200' else .0085
        add('shoulder-girdle',[[x,y,z,r,r] for x,y,z in span],slender=scale=='500')
    for piece in pieces:
        piece['bone']=piece['member']='shoulder-girdle'
        for section in piece['sections']:
            x,y,z=section['at']
            section['at']=[-y,round(x-.42,6),z]
            if 'rotation' in section:section['rotation']=[.5,-.5,.5,.5]
        i=text.index('{\n            "id": "'+piece['id']+'"',offset)
        j=sp['member_end'](text,i)
        assert json.loads(text[i:j])['bone']=='shoulder-girdle'
        new=sp['dump'](piece,5,'compact')
        text=text[:i]+new+text[j:]
        offset=i+len(new)
assert path.read_text()==old
assert json.loads(text)
path.write_text(text)
