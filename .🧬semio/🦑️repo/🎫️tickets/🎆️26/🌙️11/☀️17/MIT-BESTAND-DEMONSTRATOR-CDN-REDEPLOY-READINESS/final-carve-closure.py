import re,sys,json,collections
try: import tomllib
except ImportError: import tomli as tomllib
lock=tomllib.load(open('Cargo.lock','rb'))
pk={}
for p in lock['package']:
    pk.setdefault(p['name'],[]).append(p)
def deps(n):
    out=set()
    for p in pk.get(n,[]):
        for d in p.get('dependencies',[]): out.add(d.split(' ')[0])
    return out
def closure(roots):
    seen=set();st=list(roots)
    while st:
        n=st.pop()
        if n in seen: continue
        seen.add(n); st+=list(deps(n))
    return seen
if __name__=='__main__':
    for r in sys.argv[1:]:
        c=closure([r]); print(r,len(c))
