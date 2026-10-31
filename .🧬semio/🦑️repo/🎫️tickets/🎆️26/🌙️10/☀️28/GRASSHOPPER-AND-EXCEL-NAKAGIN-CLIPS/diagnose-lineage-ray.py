"""🔬 Diagnose actual ray disagreements without NumPy imports or rendering."""
from pathlib import Path
import importlib.util,json,math,sys
T=Path(__file__).resolve().parent;R=next(p for p in T.parents if (p/'🧰️framework').is_dir())
P=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/✂️fragments/🐍️.py'
spec=importlib.util.spec_from_file_location('diagnostic_kernel',P);K=importlib.util.module_from_spec(spec);sys.modules[spec.name]=K;spec.loader.exec_module(K)
C=json.loads(P.with_name('🧪️cases').joinpath('🔣️.json').read_text(encoding='utf8'))['lineage']
rows=tuple(K.Face(f['source'],f['polygon'],tuple(map(tuple,f['vertices'])),tuple(f['normal'])) for f in C['faces']);out=K.ordered_fragments(rows,C['towards'])
dot=lambda a,b:math.fsum(x*y for x,y in zip(a,b))
unit=lambda v:tuple(x/math.hypot(*v) for x in v)
t=unit(C['towards']);seed=tuple(int(i==min(range(3),key=lambda j:abs(t[j]))) for i in range(3));h=unit(K._cross(seed,t));matrix=(h,K._cross(t,h),t)
def transform(rows):
    return [(f.source,[tuple(dot(p,v) for v in matrix) for p in f.vertices],tuple(dot(f.normal,v) for v in matrix)) for f in rows]
def hit(rows,p,strict):
    result=[]
    for source,vs,n in rows:
        if abs(n[2])<1e-12:continue
        z=(dot(n,vs[0])-n[0]*p[0]-n[1]*p[1])/n[2]
        inside=False;boundary=False;dist=[]
        for a,b in zip(vs,vs[1:]+vs[:1]):
            e=(b[0]-a[0],b[1]-a[1]);d=(p[0]-a[0],p[1]-a[1]);cross=e[0]*d[1]-e[1]*d[0];along=dot(e,d);length=math.hypot(*e)
            eps=64*math.ulp(max(abs(v) for v in (*p,*a,*b)))
            boundary|=abs(cross)<=eps*length and -eps*length<=along<=length*length+eps*length if strict else abs(cross)<1e-10 and -1e-10<=along<=length*length+1e-10
            if (a[1]>p[1])!=(b[1]>p[1]) and a[0]+(p[1]-a[1])*e[0]/e[1]>p[0]:inside=not inside
            fraction=min(1,max(0,along/(length*length))) if length else 0
            dist.append(math.hypot(d[0]-fraction*e[0],d[1]-fraction*e[1]))
        if inside or boundary:result.append({'source':source,'depth':z,'edge_distance':min(dist),'vertices':vs,'boundary':boundary,'inside':inside})
    return result
original,painted=transform(rows),transform(out);low=[min(v[i] for _,vs,_ in original for v in vs) for i in (0,1)];high=[max(v[i] for _,vs,_ in original for v in vs) for i in (0,1)];bad=[]
for i in range(1,30):
    for j in range(1,28):
        p=[low[0]+i*(high[0]-low[0])/30,low[1]+j*(high[1]-low[1])/28]
        a,b=hit(original,p,False),hit(painted,p,False)
        if bool(a)!=bool(b):bad.append({'point':p,'old_original':a,'old_painted':b,'scaled_original':hit(original,p,True),'scaled_painted':hit(painted,p,True)})
result={'mismatches':len(bad),'first':bad[0] if bad else None};(T/'🗑️generated/lineage-ray-diagnostic.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(result),flush=True)
