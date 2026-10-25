import re,sys
def blocks(lines):
    """top-level blocks: (name,start,end) 1-based, start includes preceding doc-comment lines"""
    res=[];i=0;n=len(lines)
    decl=re.compile(r'^(?:export )?(?:async )?function (\w+)|^(?:export )?(?:const|let) (\w+)|^(?:export )?type (\w+)|^(?:export )?interface (\w+)')
    while i<n:
        m=decl.match(lines[i])
        if m:
            name=next(g for g in m.groups() if g)
            s=i
            while s>0 and lines[s-1].startswith('/**'): s-=1
            e=i
            if re.match(r'^(?:export )?(?:async )?function ',lines[i]) and not lines[i].rstrip().endswith('}'):
                e=i
                while e<n and lines[e]!='}': e+=1
            res.append((name,s+1,e+1));i=e+1
        else: i+=1
    return res
if __name__=="__main__":
    lines=open(sys.argv[1],encoding='utf-8').read().split('\n')
    for b in blocks(lines): print(*b)
