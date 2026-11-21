"""🛢️ Cut the production passage through the valve body and saddle the open flowline onto its wall."""
from copy import deepcopy
from pathlib import Path
from runpy import run_path
from math import pi,cos,sin,asin,sqrt
import json
T=Path(__file__).resolve().parent;g=T/'🗑️generated';first=run_path(str(T/'digital-well-gland.py'));CAT=first['CAT']
def surface():
    points=[];index={};faces=[]
    def vertex(xyz):
        p=tuple(round(x,8)for x in xyz)
        if p not in index:index[p]=len(points);points.append(list(p))
        return index[p]
    def quad(a,b,c,d,flip=False):
        for tri in ((a,b,c),(a,c,d)):
            if len(set(tri))==3:faces.append(list(reversed(tri))if flip else list(tri))
    def finish():
        vol=0
        for a,b,c in faces:
            x,y,z=points[a];xx,yy,zz=points[b];xxx,yyy,zzz=points[c]
            vol+=x*(yy*zzz-zz*yyy)+y*(zz*xxx-xx*zzz)+z*(xx*yyy-yy*xxx)
        if vol<0:
            for f in faces:f.reverse()
        return dict(kind='polyhedron',vertices=points,triangles=faces)
    return vertex,quad,finish
def valve(shape,rows,n,m):
    vertex,quad,finish=surface();lo=.68-.054;hi=.68+.054
    sections=[]
    for y,outer,inner in rows:
        sections.append((y,outer,inner,0))
        if y==lo:
            for j in range(1,m):
                u=pi*j/m;sections.append((.68-.054*cos(u),outer,inner,.054*sin(u)))
    rings=[]
    for y,r,ri,cut in sections:
        both=[]
        for radius in (r,ri):
            az=asin(cut/radius);both.append([vertex((-3+radius*cos(az+(2*pi-2*az)*k/n),y,radius*sin(az+(2*pi-2*az)*k/n)))for k in range(n+1)])
        rings.append(both)
    for j,(low,high) in enumerate(zip(rings,rings[1:])):
        for k in range(n):
            for side in (0,1):quad(low[side][k],high[side][k],high[side][k+1],low[side][k+1],bool(side))
        if sections[j][3] or sections[j+1][3]:
            quad(low[0][0],low[1][0],high[1][0],high[0][0])
            quad(low[0][-1],high[0][-1],high[1][-1],low[1][-1])
    for row,flip in [(rings[0],False),(rings[-1],True)]:
        for k in range(n):quad(row[0][k],row[0][k+1],row[1][k+1],row[1][k],flip)
    for key in ('profile','origin','axis'):shape.pop(key,None)
    shape.update(finish())
def pipe(n,filled=False):
    vertex,quad,finish=surface();rings=[]
    inner=0 if filled else .054
    for r,far in [(.060,False),(.060,True),(inner,True),(inner,False)]:
        rings.append([vertex((-2 if far else -3+sqrt(.10**2-(r*sin(2*pi*k/n))**2),.68+r*cos(2*pi*k/n),r*sin(2*pi*k/n)))for k in range(n)])
    for j in range(4):
        for k in range(n):quad(rings[j][k],rings[j][(k+1)%n],rings[(j+1)%4][(k+1)%n],rings[(j+1)%4][k])
    return finish()
def author(a):
    first['author'](a);reps=a['representations']
    next(q for q in a['thermal']['regions'] if q['id']=='polished-rod')['areaBasis']='total'
    a['materials'].append(dict(id='produced-fluid',title={'en':'Produced well fluid','de':'Gefördertes Bohrlochfluid'},emissivity=.93,solarAbsorptivity=.82,densityKgM3=850,conductivityWmK=.14,specificHeatJKgK=1900,source='authored-estimate'))
    for d,v in a['volumes'].items():
        sols={s['id']:s for s in v['body']};n={'50':32,'100':24,'200':20,'500':16}[d];m=n//2
        low=.68-.054;hi=.68+.054
        window=[(low,.1,.065),(hi,.1,.065)]
        upper=[(.94,.1,.065),(1.,.1,.0285)]
        bottom=[(0,.17,.070),(.5,.17,.070),(.5,.2,.070),(.53,.2,.070),(.53,.1,.065)]
        seat=[(1.,.08,.0285),(1.22,.08,.0285),(1.222,.0793,.0285),(1.222,.0793,.039),(1.28,.06,.039)]
        if 'wellhead-base' in sols:
            for p in sols['wellhead-base']['profile']:
                if p[1]==.0285:p[1]=.070
        if d=='50':valve(sols['wellhead-valve'],[(.53,.1,.065),*window,*upper],n,m)
        elif d=='100':valve(sols['wellhead-top'],[(.53,.1,.065),*window,*upper,*seat],n,m)
        else:valve(sols['wellhead'],[*bottom,*window,*upper,*seat],n,m)
        branch=dict(id='flowline',bone='base',material='painted-steel',role='body',tone=.55,**pipe(n))
        if 'flowline' in sols:sols['flowline'].clear();sols['flowline'].update(branch)
        else:
            v['body'].append(branch)
            for view,shapes in reps[d]['geometry'].items():
                contour=deepcopy(next(q for q in reps['50']['geometry'][view]if q['id']=='flowline'))
                contour['penMm']=.20;shapes.append(contour);shapes.sort(key=lambda q:q['z'])
        v['thermal'][:]=[q for q in v['thermal'] if q['id']!='th-flowline']
        v['thermal'].append(dict(id='th-flowline',bone='base',material='painted-steel',tone=.55,role='body',region='wellhead',sectionFaces=['+u'],**pipe(n,True)))
        fluid=dict(id='well-fluid',kind='revolve',bone='base',material='produced-fluid',tone=.36,role='body',region='wellhead',axis=[0,1,0],origin=[-3,0,0],profile=[[0,.025],[0,.07],[.53,.07],[.53,.065],[.94,.065],[1.,.0285],[1.222,.0285],[1.222,.025],[0,.025]])
        v['thermal'].append(fluid)
        v['structure'].append(dict(id='member-flowline',of='flowline',member='base',part='member'))
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (g/'digital-well-complete.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
