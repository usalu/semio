"""🩼 Author the crutch tubing, adjustment bores, locking detents and orthopedic boot straps."""
from collections import Counter
from copy import deepcopy
from math import asin,cos,hypot,pi,sin
from pathlib import Path
import json
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/person-on-crutches/*json'))
def tubing(axis,outer,inner,limits,holes,steps,quarter):
    rows=[(limits[0],0.)]
    for station in holes:
        rows.extend((station-.004*cos(pi*j/steps),.004*sin(pi*j/steps)) for j in range(steps+1))
    rows.append((limits[1],0.))
    vertices=[];indices={};pair={};faces=[];edges=Counter()
    def vertex(s,r,angle):
        q=tuple(round(x,9) for x in (axis[0]*s-axis[2]*r*sin(angle),r*cos(angle),axis[2]*s+axis[0]*r*sin(angle)))
        if q not in indices:indices[q]=len(vertices);vertices.append(q)
        return indices[q]
    def angles(delta,k):
        lo,hi=[(delta,pi/2),(pi/2,pi-delta),(pi+delta,3*pi/2),(3*pi/2,2*pi-delta)][k]
        return [lo+(hi-lo)*j/quarter for j in range(quarter+1)]
    previous=None
    for s,half in rows:
        current=[]
        for k in range(4):
            exterior=[vertex(s,outer,t) for t in angles(asin(half/outer),k)]
            interior=[vertex(s,inner,t) for t in angles(asin(half/inner),k)]
            current.append(exterior);pair.update(zip(exterior,interior))
            if previous:
                for j in range(quarter):
                    a,b=previous[k][j:j+2];c,d=exterior[j+1],exterior[j]
                    for tri in ((a,b,c),(a,c,d)):
                        faces.append(tri)
                        for edge in zip(tri,tri[1:]+tri[:1]):edges[edge]+=1
        previous=current
    surface=list(faces);faces.extend(tuple(pair[i] for i in reversed(tri)) for tri in surface)
    for a,b in edges:
        if (b,a) not in edges:faces.extend([(b,a,pair[a]),(b,pair[a],pair[b])])
    return dict(kind='polyhedron',vertices=list(map(list,vertices)),triangles=list(map(list,faces)))
def author(a):
    a['materials'].append(dict(id='detent-stainless',title={'en':'Passivated stainless detent pin','de':'Passivierter Edelstahl-Rastbolzen'},emissivity=.24,solarAbsorptivity=.35,densityKgM3=7900,conductivityWmK=16,specificHeatJKgK=500,source='authored-estimate'))
    bore_walls=4*pi*(.0097*(.32+.012)+.008*(.856-.32))
    nominal=.16+bore_walls
    frame=next(q for q in a['thermal']['regions'] if q['id']=='crutch-frame')
    frame['areaM2']=round(nominal,5)
    for pose in a['poses']:pose['thermal']['regions']['crutch-frame']['exposedFraction']=round(.16/nominal,6)
    a['thermal']['assumptions'].append({'en':'The paired aluminium frames have 0.16 m² facing the room and approximately 0.094 m² inside the perforated tubes. Direct ambient exchange uses the outward-facing area. The bore estimate excludes the 148 mm of overlapping upper-tube wall.','de':'Beide Aluminiumrahmen haben 0,16 m² zum Raum gerichtete Fläche und rund 0,094 m² innerhalb der gelochten Rohre. Der direkte Wärmeaustausch mit der Umgebung erfolgt über die Außenflächen. Die Bohrungsfläche berücksichtigt die 148 mm überdeckte Innenwand des Oberrohrs nicht.'})
    for d,v in a['volumes'].items():
        steps,quarter={'50':(8,6),'100':(6,4),'200':(4,3),'500':(4,2)}[d]
        for side,sign in [('l',1),('r',-1)]:
            bone='crutch-'+side;axis=[.99631/hypot(.99631,.08579),0,sign*.08579/hypot(.99631,.08579)]
            upper=next(q for q in v['body'] if q['id']=='crutch-tube-'+side)
            base={k:upper[k] for k in ['bone','material','role','tone']}
            items=[]
            if upper['kind']=='loft':
                bracket=deepcopy(next(q for q in a['volumes']['50']['body'] if q['id']=='crutch-bracket-'+side))
                items.append(bracket)
            for id,geo in [('crutch-tube',tubing(axis,.011,.0097,(-.012,.468),[.42],steps,quarter)),('crutch-lower',tubing(axis,.0095,.008,(.32,.856),[.42,.47,.52,.57,.62,.67,.72],steps,quarter))]:
                items.append(dict(base,id=id+'-'+side,**geo))
                if d!='50':items[-1]['slender']=True
            clip=dict(base,id='crutch-clip-'+side,kind='revolve',tone=.32,axis=axis,origin=[0,0,0],profile=[[.452,.01105],[.452,.0138],[.478,.0138],[.484,.011],[.484,.0096],[.468,.0096],[.468,.01105],[.452,.01105]])
            lock=dict(base,id='crutch-detent-'+side,kind='revolve',material='detent-stainless',tone=.61,axis=[0,1,0],origin=[.42*x for x in axis],profile=[[-.0123,0],[-.0123,.0023],[-.0116,.0038],[.0116,.0038],[.0123,.0023],[.0123,0]])
            items.extend([clip,lock])
            ids={q['id'] for q in items}
            gone={q['id'] for q in v['body'] if q['id'].startswith('crutch-hole-') and q['bone']==bone}
            v['body'][:]=[q for q in v['body'] if q['id'] not in ids|gone]+items
            v['thermal'][:]=[q for q in v['thermal'] if q.get('of') not in gone]
            for item in items:
                name=item['id']
                if not any(q.get('of')==name for q in v['thermal']):v['thermal'].append(dict(id='th-'+name,of=name,region='crutch-frame'))
                if not any(q.get('of')==name for q in v['structure']):v['structure'].append(dict(id='member-'+name,of=name,member=bone,part='member'))
    for view,dim in [('elevation',0),('front',1)]:
        for q in a['representations']['100']['geometry'][view]:
            if q['id'].startswith('boot-strap-') and q['kind']=='poly':
                value=[p[dim] for p in q['points']];center=(max(value)+min(value))/2
                for p in q['points']:p[dim]=round(center+.94*(p[dim]-center),5)
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-crutch-tubes.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
