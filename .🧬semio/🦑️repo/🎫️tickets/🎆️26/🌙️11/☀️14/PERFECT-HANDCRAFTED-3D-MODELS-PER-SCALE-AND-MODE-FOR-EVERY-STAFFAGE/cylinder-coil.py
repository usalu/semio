"""🌀 Routes the isolated primary circuit and the dry sensor pocket into the storage vessel."""
from pathlib import Path
import json,math,copy
t=Path(__file__).resolve().parent;out=t/'🗑️generated'/'rotor';p=out/'hot-water-cylinder.json';doc=json.loads(p.read_bytes());a=doc['asset']
def unit(x):m=math.sqrt(sum(v*v for v in x));return [v/m for v in x]
def cross(x,y):return [x[1]*y[2]-x[2]*y[1],x[2]*y[0]-x[0]*y[2],x[0]*y[1]-x[1]*y[0]]
def cubic(points,t):return [sum(v[j]*w for v,w in zip(points,[(1-t)**3,3*(1-t)**2*t,3*(1-t)*t*t,t**3]))for j in range(3)]
def coil(d):
    h1=.9227;h0=.5102;r=.16;turns=20/3;per={50:30,100:24,200:18,500:12}[d];n={50:12,100:10,200:8,500:6}[d];tube=[]
    c1=[[.235,h1,0],[.205,h1,0],[.119,h1+.002,-.116],[.08,h1,-r*math.sqrt(.75)]]
    c2=[[.08,h0,r*math.sqrt(.75)],[.119,h0-.002,.116],[.205,h0,0],[.235,h0,0]]
    tube.extend(cubic(c1,i/6) for i in range(6))
    count=round(turns*per);tube.extend([r*math.cos(-math.pi/3-i*math.tau*turns/count),h1+(h0-h1)*i/count,r*math.sin(-math.pi/3-i*math.tau*turns/count)]for i in range(count+1))
    tube.extend(cubic(c2,i/6)for i in range(1,7))
    verts=[];tri=[]
    for i,pt in enumerate(tube):
        tangent=unit([y-x for x,y in zip(tube[max(0,i-1)],tube[min(i+1,len(tube)-1)])])
        if i==0:tangent=[-1,0,0]
        if i==len(tube)-1:tangent=[1,0,0]
        u=unit(cross(tangent,[0,1,0]));v=cross(tangent,u)
        for radius in [.012,.0085]:
            for j in range(n):
                w=j*math.tau/n;verts.append([round(pt[k]+radius*(math.cos(w)*u[k]+math.sin(w)*v[k]),7) for k in range(3)])
    def face(a,b,c,d):tri.extend([[a,b,c],[a,c,d]])
    for i in range(len(tube)-1):
        off=i*2*n;end=off+2*n
        for j in range(n):
            k=(j+1)%n;face(off+j,off+k,end+k,end+j);face(off+n+j,end+n+j,end+n+k,off+n+k)
    last=(len(tube)-1)*2*n
    for j in range(n):
        k=(j+1)%n;face(j,n+j,n+k,k);face(last+j,last+k,last+n+k,last+n+j)
    return {'id':'charging-coil','kind':'polyhedron','bone':'connections','material':'enamelled-steel','role':'detail','tone':.37,'vertices':verts,'triangles':tri}
for d,s in a['volumes'].items():
    s['body'].append(coil(int(d)))
    q=copy.deepcopy(next(q for q in s['body']if q['id']=='stub-1'));q.update(id='sensor-pocket',role='detail',origin=[-.235,.7852,0],axis=[1,0,0],profile=[[0,.0085],[0,.012],[.049,.012],[.049,0],[.046,0],[.046,.0085],[0,.0085]])
    s['body'].append(q)
    for key in ['charging-coil','sensor-pocket']:s['structure'].append({'id':'member-'+key,'of':key,'member':'connections','part':'member'})
    s['decisions'].append({'en':'The right tappings feed a separate charging coil with six and two-thirds turns. The left middle tapping receives a closed immersion pocket; its accessible instrument bore remains dry.','de':'Die rechten Anschlüsse versorgen eine getrennte Heizwendel mit sechs und zwei Drittel Windungen. Der mittlere linke Anschluss trägt eine geschlossene Tauchhülse mit trockenem, von außen zugänglichem Fühlerraum.'})
p.write_text(json.dumps(doc,ensure_ascii=False))
