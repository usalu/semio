import sys,collections,re,os,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from load import *
R='/Users/Shared/semio/'
SPEC=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+|require\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
def edges(files):
    fs=set(files);out={}
    for f in files:
        if not re.search(r'\.(ts|mjs|js|cjs|mts|tsx)$',f):continue
        try:src=open(R+f,encoding='utf8').read()
        except Exception:continue
        dst=set()
        for m in SPEC.finditer(src):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            for c in (p,p[:-3]+'.ts' if p.endswith('.js') else p,p.replace('.mjs','.d.mts')):
                if c in fs:dst.add(c);break
        out[f]=dst
    return out
def roots(tag,TH=21):
    b=load(tag);fo=collections.defaultdict(set)
    for r,s in b['own'].items():
        for f in s:fo[f].add(r)
    U=[f for f,v in fo.items() if len(v)>=TH];E=edges(U);inc=collections.Counter()
    for f,ds in E.items():
        for d in ds:
            if d!=f:inc[d]+=1
    rs=[f for f in U if inc[f]==0 and re.search(r'\.(ts|mjs)$',f)]
    def reach(r):
        seen={r};st=[r]
        while st:
            x=st.pop()
            for d in E.get(x,()):
                if d not in seen:seen.add(d);st.append(d)
        return seen
    return fo,U,sorted(((len(fo[r]),len(reach(r)),r) for r in rs),reverse=True)
if __name__=='__main__':
    fo,U,rows=roots(sys.argv[1])
    print(len(U),'shared files',len(rows),'code roots')
    for n,k,r in rows:print(n,k,r.replace('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/','LIB/')[-100:])
