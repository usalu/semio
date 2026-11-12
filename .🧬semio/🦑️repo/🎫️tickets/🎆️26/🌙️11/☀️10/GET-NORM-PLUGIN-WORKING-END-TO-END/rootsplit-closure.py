import re,os,sys,json
R=os.getcwd()
pat=re.compile(r'(?:from\s+|import\s*\(\s*|import\s+)["\'](\.[^"\']+)["\']')
cache={}
def imps(f):
    if f in cache: return cache[f]
    try: s=open(f,encoding='utf8').read()
    except: cache[f]=[];return []
    out=[]
    for m in pat.finditer(s):
        p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
        if os.path.isdir(p): p=os.path.join(p,'🟦️.ts')
        if os.path.exists(p) and p.endswith(('.ts','.tsx','.js','.mjs')): out.append(p)
    cache[f]=out;return out
def closure(f):
    seen={f};st=[f]
    while st:
        x=st.pop()
        for y in imps(x):
            if y not in seen: seen.add(y);st.append(y)
    return seen
if __name__=='__main__':
    for f in sys.argv[1:]:
        c=closure(os.path.abspath(f)); print(len(c),f[-80:])
