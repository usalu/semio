# 🧮️ Splits the reconcile module file into top-level items (with attached docs/attrs) and prints their ranges.
import re,sys
R='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/'
def items(path):
    lines=open(path).read().split('\n')
    out=[];i=0;n=len(lines)
    starts=[]
    for idx,l in enumerate(lines):
        if l=='' or l.startswith(' ') or l.startswith('\t') or l.startswith('}') or l.startswith(')') or l.startswith(']'): continue
        starts.append(idx)
    # group consecutive column-0 header lines (docs/attrs) with their item
    k=0
    blocks=[]
    cur=None
    for idx in starts:
        l=lines[idx]
        is_pre=l.startswith('///') or l.startswith('//!') or l.startswith('#[') or l.startswith('// ') or l.startswith('//#') and False
        if l.startswith('//#region') or l.startswith('//#endregion'):
            if cur is not None: blocks.append(cur); cur=None
            blocks.append(('marker',idx,idx,l)); continue
        if cur is None: cur=[idx,None]
        if is_pre: continue
        # item line
        cur[1]=idx
        blocks.append(('item',cur[0],idx,l)); cur=None
    res=[]
    for b in blocks:
        res.append(b)
    # compute ends: item ends before next block start (strip trailing blank lines)
    full=[]
    for j,b in enumerate(res):
        end=(res[j+1][1]-1) if j+1<len(res) else n-1
        while end>b[1] and lines[end]=='' : end-=1
        full.append((b[0],b[1]+1,end+1,b[3]))
    return lines,full
if __name__=='__main__':
    lines,full=items(R+'♻️reconcile/🦀️.rs')
    for k,s,e,t in full: print(f'{k[:1]} {s:5d}-{e:5d} {e-s+1:4d} {t[:110]}')
