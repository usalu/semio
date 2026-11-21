"""🪵 Hand-author the lift screws, captive nut crossheads and slotted slide runners of the extension table."""
from pathlib import Path
from runpy import run_path
import json
import math

HERE = Path(__file__).resolve().parent
CATALOG = next(HERE.parents[6].glob("🎓*/*/*staffage/🗂*/*/dining-table/🔣️.json"))
SOURCE = HERE / "volumes/furniture/dining-table.json"
STATIONS = (.64, .96)
FRAME_Y = .6775
LOD = run_path(str(HERE / "digital-leaf-lod.py"))
NOTE = {"en": "Two steel crossheads on the stationary slide bars carry four captive lifting nuts. Non-rotating screw posts travel with the leaf. Its weight passes through the thrust collars and crossheads into the slide bars and both timber aprons.", "de": "Zwei stählerne Querträger an den feststehenden Auszugsstegen tragen vier gefasste Hubmuttern. Verdrehsichere Spindeln fahren mit der Einlegeplatte. Über die Druckbunde und Querträger gelangt die Last in die Auszugsstege und beide Holzzargen."}


def base(sid, kind, bone):
    return dict(id=sid, kind=kind, bone=bone, material="steel-slides", role="body", tone=.36)


def turn(sid, bone, origin, section):
    return dict(base(sid, "revolve", bone), axis=[0, 1, 0], origin=origin, profile=section,
                repeat=dict(count=2, translation=[0, 0, .49]))


def ring(cx, cz, radius, sides):
    return [[round(cz+radius*math.cos(2*math.pi*i/sides), 7), round(cx+radius*math.sin(2*math.pi*i/sides), 7)] for i in range(sides)]


def pocket(lo, hi, sides):
    radius=.013
    return [[round(center+radius*math.cos(angle),7),round(radius*math.sin(angle),7)]
            for center,start in [(hi-radius,-math.pi/2),(lo+radius,math.pi/2)]
            for angle in [start+math.pi*j/sides for j in range(sides+1)]]


def hardware(scale):
    out = []
    for i, world in enumerate(STATIONS):
        x = round(world-.30, 4)
        width = .036
        rim = .0136
        sides = {50:32,100:24,200:16,500:12}[scale]
        ends = [[-.31,-width/2],[.31,-width/2],[.31,width/2],[-.31,width/2]]
        out.append(dict(base(f"lift-crosshead-{i}", "extrude", "frame"), plane="zx", profile=ends,
                        holes=[ring(0,z,rim,sides) for z in [-.245,.245]] + [pocket(lo,hi,6 if scale==50 else 3) for lo,hi in [(-.30,-.27),(-.22,-.084),(-.075,.075),(.084,.22),(.27,.30)] if scale<=200], **{"from": -.0415, "to": -.0235},
                        transform=dict(translation=[x,0,0])))
        for side,s in [("near",-1),("far",1)]:
            end={50:.365,100:.357,200:.34,500:.363}[scale]
            yz=[[-.0415,.31],[-.0415,.319],[-.005,.353],[-.005,end],[.005,end],[.005,.347],[-.0235,.319],[-.0235,.31]]
            if scale>50: yz=[[-.0415,.31],[-.0415,.32],[-.005,end],[.005,end],[.005,min(.348,end-.005)],[-.0235,.315],[-.0235,.31]]
            out.append(dict(base(f"lift-bracket-{i}-{side}","extrude","frame"), plane="yz", profile=[[y,round(s*z,6)] for y,z in yz], **{"from":round(x-width/2,4),"to":round(x+width/2,4)}))
        nut=[[-.0475,.008],[-.0475,.020],[-.0415,.020],[-.0415,.013],[-.0235,.013],[-.0235,.026],[-.0195,.029],[-.0135,.029],[-.0105,.025],[-.0105,.008],[-.0475,.008]] if scale==50 else [[-.0445,.008],[-.0445,.019],[-.0415,.019],[-.0415,.013],[-.0235,.013],[-.0235,.027],[-.0105,.027],[-.0105,.008],[-.0445,.008]]
        if scale>=200: nut=[[-.0415,.008],[-.0415,.013],[-.0235,.013],[-.0235,.025],[-.0105,.025],[-.0105,.008],[-.0415,.008]]
        out.append(turn(f"lift-nut-{i}","frame",[x,0,-.245],nut))
        rod=[[-.090,0],[-.09,.0065],[-.0885,.008],[-.006,.008],[-.004,.017],[-.0015,.019],[0,.019],[0,0]] if scale==50 else [[-.09,0],[-.09,.008],[-.005,.008],[-.005,.018],[0,.018],[0,0]]
        out.append(turn(f"lift-screw-{i}","leaf",[round(world-.50,4),0,-.245],rod))
    return out


def author(a):
    for k in ['50','100','200','500']:
        v=a['volumes'][k]
        if int(k)>=200 and 'representations' in a: LOD['drawing'](a,int(k))
        if k=='500': LOD['coarse'](v)
        for group,prefix in [('body','lift-'),('thermal','th-lift-'),('structure','st-lift-')]:
            v[group]=[s for s in v[group] if not s['id'].startswith(prefix)]
        if k=='50':
            for b in v['body']:
                if not b['id'].startswith('runner-'): continue
                s=1 if b['id'].endswith('-l') else -1
                outline=[[-.0635,.363],[-.070,.363],[-.070,.375],[-.045,.375],[-.045,.363],[-.0515,.363],[-.0515,.365],[-.047,.365],[-.047,.373],[-.068,.373],[-.068,.365],[-.0635,.365]]
                b['profile']=[[y,s*z] for y,z in outline]
                b.pop('holes',None)
        for b in hardware(int(k)):
            v['body'].append(b)
            v['thermal'].append(dict(id='th-'+b['id'],of=b['id'],region='slide-rails',**({'layer':'rail-steel'} if k=='50' else {})))
            v['structure'].append(dict(id='st-'+b['id'],of=b['id'],member=b['bone'],part='joint' if 'nut' in b['id'] else 'member'))
        if NOTE not in v['decisions']:v['decisions'].append(NOTE)
    if 'structure' in a:
        if NOTE not in a['structure']['assumptions']:a['structure']['assumptions'].append(NOTE)
    if 'rig' in a:
        masses={'frame':(7.45746, -.01764),'leaf':(11.85871,.01211),'top-left':(22.668,-.06811),'top-right':(22.668,-.06811)}
        for b in a['rig']['bones']:
            if b['id'] in masses:b['mass']['kg'],b['mass']['comY']=masses[b['id']]
        a['dimensions']['massKg']=round(sum(m[0] for m in masses.values()),5)
        for v in a['volumes'].values():
            for part in v['structure']:
                if part.get('part')=='mass' and part.get('bone') in masses:part['transform']['translation'][1]=masses[part['bone']][1]
        rail=next(r for r in a['thermal']['regions'] if r['id']=='slide-rails')
        rail['areaM2']=.405
        rail['title']={'en':'Slide rails and lift mechanism','de':'Auszugsschienen und Hubmechanik'}
        for pose in a['poses']:
            pose['thermal']['regions']['slide-rails']['exposedFraction']=.73 if pose['id'] in ['closed','edge-test-load'] else .88
    else:
        for v in a['volumes'].values():
            for part in v['structure']:
                if part.get('part')=='mass' and part['bone'] in ['frame','leaf','top-left','top-right']:
                    part['transform']['translation'][1]={'frame':-.01764,'leaf':.01211,'top-left':-.06811,'top-right':-.06811}[part['bone']]


def apply():
    replace=run_path(str(HERE/'r5-range-gaps-lib.py'))
    for path in [CATALOG,SOURCE]:
        before=path.read_text()
        doc=json.loads(before)
        asset=doc.get('asset',doc)
        original={key:json.dumps(value,ensure_ascii=False) for key,value in asset.items()}
        author(asset)
        updated=before
        prefix='    ' if path==CATALOG else '  '
        for key,value in asset.items():
            if original[key]==json.dumps(value,ensure_ascii=False):continue
            import re
            match=re.search('^'+(prefix if path==CATALOG else ' *')+'"'+key+'"\\s*:\\s*',updated,re.M)
            if not match:raise ValueError((path,key))
            start=match.end();_,length=json.JSONDecoder().raw_decode(updated[start:])
            updated=updated[:start]+replace['render'](value,len(prefix),2)+updated[start+length:]
        assert json.loads(updated)==doc
        if path.read_text()!=before:raise RuntimeError('Asset changed during authoring')
        if updated!=before:path.write_text(updated)

if __name__=='__main__':apply()
