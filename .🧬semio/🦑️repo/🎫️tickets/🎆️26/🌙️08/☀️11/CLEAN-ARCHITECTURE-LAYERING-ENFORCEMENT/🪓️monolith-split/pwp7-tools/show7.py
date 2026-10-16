import sys; sys.path.insert(0,'.')
from rw7 import *
ms=methods(); T=Table(ms)
for m in ms:
    if m['recv'] is None and m['crate'] not in('app','composer-api'):
        g=[]
        if re.search(r'\bA\b',m['text']): g.append('A')
        if re.search(r'\bM\b',m['text']): g.append('M')
        T.free[m['name']]='::<%s>'%', '.join(g) if g else ''
def conv(name, crate=None):
    for m in ms:
        if m['name']==name and (crate is None or m['crate']==crate):
            flags=[]
            cview=(m['recv']=='ref' and m['crate'] in VIEW)
            return rewrite(whole_borrows(m['text']),dict(crate=m['crate'],view=cview,name=m['name']),T,flags),flags
if __name__=='__main__':
    for n in sys.argv[1:]:
        r,f=conv(n)
        print(r); print(f)
