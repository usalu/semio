import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/weeping-willow/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
def span(pts,y):
    xs=[]
    n=len(pts)
    for i in range(n):
        (x1,y1),(x2,y2)=pts[i],pts[(i+1)%n]
        if (y1<=y<y2) or (y2<=y<y1):
            xs.append(x1+(x2-x1)*(y-y1)/(y2-y1))
    return (min(xs),max(xs)) if len(xs)>=2 else None
def strands(s,n,levels):
    pts=s['points']; ys=[p[1] for p in pts]; lo,hi=min(ys),max(ys)
    out=[]
    for k in range(n):
        line=[]
        for j in range(levels+1):
            y=lo+(hi-lo)*(0.04+0.92*j/levels)
            sp=span(pts,y)
            if sp is None: continue
            u=(k+1)/(n+1)
            line.append([round(sp[0]+(sp[1]-sp[0])*u,4),round(y,4)])
        if len(line)>=3: out.append(line)
    return out
for sc,(n,levels) in {'50':(3,9),'100':(2,6)}.items():
    r=a['representations'][sc]; g=r['geometry']['front']
    new=[]
    for s in g:
        new.append(s)
        if s['id'].startswith('f-c') and s['kind']=='poly':
            for k,line in enumerate(strands(s,n,levels),1):
                new.append({"id":s['id']+'-s%d'%k,"bone":s['bone'],"role":"detail","z":round(s['z']+0.0005,4),"filled":False,"tone":s.get('tone',0.45),"kind":"poly","points":line,"closed":False})
    r['geometry']['front']=new
    t=r['decisions']['silhouette']
    t['en']+=' Every leaf curtain in front carries '+str(n)+' strand lines inside its mass, like the strands of the side view, so the front crown is not a bare outline.'
    t['de']+=' Jeder Blattvorhang trägt in der Vorderansicht '+str(n)+' Strähnenlinien in seiner Masse, wie die Strähnen der Seitenansicht, sodass die Frontkrone keine bloße Kontur ist.'
write_json_atomic(f,d)
print('ok')
