"""♻️ Cycle check of a plan: no added producer edge D -> P may close a cycle (P must not reach D through normal or build edges of the current graph). `python3 cycle_check.py <tag>`"""
from hubenv import *
tag=sys.argv[1]
plan=json.load(open(os.path.normpath(T+'/../staged/plans/%s.json'%tag)))
_,I2L,I2D,LIB=make_world()
fwd={}
def down(p):
    if p in fwd: return fwd[p]
    seen=set();st=[p]
    while st:
        x=st.pop()
        for y in E_.get(x,()):
            if y not in seen: seen.add(y);st.append(y)
    fwd[p]=seen;return seen
bad=0;n=0
for name,c in plan['crates'].items():
    d=c.get('dir')
    for i in c.get('owners_prod',[])+c.get('owners_test',[]):
        p=I2D[i];n+=1
        if p==d or d in down(p): bad+=1;print('CYCLE',name,'->',C[p]['name'])
print(tag,'added edges',n,'cycles',bad)
