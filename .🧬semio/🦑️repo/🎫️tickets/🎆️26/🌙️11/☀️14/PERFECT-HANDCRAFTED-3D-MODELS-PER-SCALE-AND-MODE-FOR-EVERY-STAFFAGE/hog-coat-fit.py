"""🦔 Model four divergent, rooted spines per fascicle over the dome, saddle and drawstring skirts."""
from pathlib import Path
from copy import deepcopy
from math import ceil, sqrt
import json
import sys

T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*architecture/*staffage/*catalog/*animals/hedgehog/*.json'))
density={50:(1,1,1),100:(.74,.70,.62),200:(.53,.55,.48),500:(.36,.4,.32)}
def plus(a,b,k=1):
    return [x+k*y for x,y in zip(a,b)]
def dot(a,b):
    return sum(x*y for x,y in zip(a,b))
def cross(a,b):
    return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def unit(a):
    length=sqrt(dot(a,a))
    return [x/length for x in a]
def model(m):
    origin=m['sections'][0]['at']
    axis=unit(plus(m['sections'][-1]['at'],origin,-1))
    u=unit(cross(axis,[0,0,1] if abs(axis[2])<.9 else [0,1,0]));v=cross(axis,u)
    def point(x,y,z):
        return [round(c,7) for c in plus(plus(plus(origin,u,x),v,y),axis,z)]
    if m['id'] in ('sp-d-196-0','sp-d-196-6'):
        origin=[origin[0]-.002,origin[1],origin[2]-(.003 if origin[2]>0 else -.003)]
    pitch=.0045
    vertices=[point(x*pitch,y*pitch,0) for y in [-1,0,1] for x in [-1,0,1]]
    triangles=[]
    for j,(x,y,length) in enumerate([(-1,-1,.0205),(1,-1,.0175),(-1,1,.0187),(1,1,.0214)]):
        ix=j%2;iy=j//2;root=3*iy+ix
        base=[root,root+1,root+4,root+3];mid=[]
        cx=x*pitch*.62;cy=y*pitch*.62
        for dx,dy in [(-1,-1),(1,-1),(1,1),(-1,1)]:
            mid.append(len(vertices));vertices.append(point(cx+.00083*dx,cy+.00083*dy,.007))
        tip=len(vertices);vertices.append(point(cx*1.65,cy*1.48,length))
        for i in range(4):
            h=(i+1)%4
            triangles.extend([[base[i],base[h],mid[h]],[base[i],mid[h],mid[i]],[mid[i],mid[h],tip]])
    rim=[0,1,2,5,8,7,6,3]
    lower=len(vertices)
    for k in rim:
        vertices.append([round(c,7) for c in plus(vertices[k],axis,-.0015)])
    center=len(vertices);vertices.append(point(0,0,-.0015))
    for i,k in enumerate(rim):
        h=(i+1)%len(rim)
        triangles.extend([[k,lower+i,lower+h],[k,lower+h,rim[h]],[center,lower+h,lower+i]])
    return dict(id='fascicle-'+m['id'][3:],kind='polyhedron',bone=m['bone'],material='spines',role='body',tone=.46+.055*(int(m['id'].split('-')[-1])%3),vertices=vertices,triangles=triangles)
def author(a):
    fine=a['volumes']['50'];tufts=[m for m in fine['body'] if m['id'].startswith('sp-')]
    if not tufts:
        assert any(m['id'].startswith('fascicle-') for m in fine['body'])
        return
    skins={m['id']:m for m in fine['body'] if m['id'] in ['saddle','dome','dome-front'] or m['id'].startswith('bowl-')}
    face={s['id']:s for s in fine['body'] if s['id'] in ('neck','head','nose','jaw','eye-n','ear-n','eye-f','ear-f')}
    rings={}
    for m in tufts:
        rings.setdefault(m['id'].rsplit('-',1)[0],[]).append(model(m))
    for scale,counts in density.items():
        vol=a['volumes'][str(scale)]
        body=[deepcopy(skins.get(s['id'],s)) for s in vol['body'] if not s['id'].startswith('sp-')]
        thermal=[s for s in vol['thermal'] if not s.get('of','').startswith('sp-')]
        if scale>50:
            body=[s for s in body if s['id'] not in face]+list(face.values())
            thermal=[s for s in thermal if s.get('of') not in face]+[s for s in fine['thermal'] if s.get('of') in face]
        for name,row in rings.items():
            fraction=counts[0 if name.startswith('sp-d-') else 1 if name.startswith('sp-s-') else 2]
            n=ceil(len(row)*fraction)
            for j in range(n):
                m=row[int((j+.5)*len(row)/n)]
                skirt=m['bone'].startswith('coat-')
                body.append(m);thermal.append(dict(id='th-'+m['id'],of=m['id'],region='spines-skirt' if skirt else 'spines-dome',layer='skirt-spines' if skirt else 'dome-spines'))
        vol['body']=body;vol['thermal']=thermal
        n=sum(s['id'].startswith('fascicle-') for s in body)
        vol['decisions'][0]=dict(en=f'The physical spine coat comprises {n} four-spine fascicles. Slender shafts open from a shared basal patch on the rump and shoulder domes, over the back joint and on four independently contracting coat skirts. Longitudinal stations preserve their direction in the foraging stance and open into a radiating defensive mantle when rolled. The four digital scales retain the physical shaft length and reduce the density of fascicles with distance.',de=f'Das Stachelkleid umfasst {n} Büschel aus je vier Stacheln. Die schlanken Schäfte spreizen sich aus einem gemeinsamen Ansatz auf der Kruppen- und Schulterkuppel, über dem Rückengelenk und auf den vier unabhängig beweglichen Mantelschürzen. Die Längsreihen folgen beim Suchen der Fellrichtung und bilden beim Einrollen einen radialen Schutzmantel. Mit zunehmendem Darstellungsmaßstab sinkt die Büscheldichte bei gleichbleibender Schaftlänge.')
if __name__=='__main__':
    doc=json.loads(Path(sys.argv[1]).read_text());author(doc['asset']);Path(sys.argv[2]).write_text(json.dumps(doc,separators=(',',':'),ensure_ascii=False))
