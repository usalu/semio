import sys; sys.path.insert(0,'.')
from finish7 import *
def scan(W, writes):
    # output file of each member
    def out_file(m):
        c=m['crate']
        if c in REHOMED:
            return ROOT_OF[c]+'/📥️staging/'+STAGE[c][DOMAIN_OF[m['name']]]+'/🦀️.rs'
        if m['name'] in LADDER_EXTRA: return LADDER_EXTRA[m['name']]
        return m['file']
    res=[]
    MK={f:mask(txt) for f,txt in writes.items() if f.endswith('.rs')}
    for m in W.ms:
        if m['crate'] in ('composer-api','app'): continue
        if m['vis'] not in ('',): continue
        of=out_file(m)
        name=m['name']
        callers=set()
        for f,mk in MK.items():
            if f==of: continue
            if re.search(r'(?:\.|::)\s*'+re.escape(name)+r'\s*(?:::<[^>]*>)?\s*\(', mk) or (m['recv'] is None and re.search(r'(?<![\w.])'+re.escape(name)+r'\s*(?:::<[^>]*>)?\s*\(', mk)):
                callers.add(f)
        if callers: res.append((m['crate'],name,m['recv'],of,sorted(callers)))
    return res
if __name__=='__main__':
    W,writes,deletes=finish()
    for r in scan(W,writes): print(r[0],r[1],r[2],'| defined in',r[3].split('/')[-2],'| callers',[c.split('/')[-2] for c in r[4]])
