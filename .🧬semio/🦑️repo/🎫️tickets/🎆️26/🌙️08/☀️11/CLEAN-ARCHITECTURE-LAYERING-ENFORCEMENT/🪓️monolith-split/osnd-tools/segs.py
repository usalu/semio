"""🧩 Lossless top-level segmentation of a Rust file: items (with their leading comments and blanks), region markers and floating text."""
import re
from rlex2 import lex2
DEF=[('struct',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?struct\s+(\w+)')),('enum',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?enum\s+(\w+)')),
 ('fn',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+(\w+)')),
 ('const',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?const\s+(\w+)')),('static',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?static\s+(?:mut\s+)?(\w+)')),
 ('type',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?type\s+(\w+)')),('trait',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?trait\s+(\w+)')),
 ('mod',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)')),('macro',re.compile(r'^macro_rules!\s*(\w+)')),
 ('impl',re.compile(r'^(?:unsafe\s+)?impl\b')),('use',re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?use\b')),('extern',re.compile(r'^extern\s+crate\b'))]
REGION=re.compile(r'^\s*//\s*#(region|endregion)\b(.*)$')
def parse(path_or_lines):
    if isinstance(path_or_lines,str):
        t=open(path_or_lines,encoding='utf-8').read()
        L=t.split('\n')
        if L and L[-1]=='':L=L[:-1];trail=True
        else:trail=False
        L=[l+'\n' for l in L]
    else:L=path_or_lines;trail=True
    infos=lex2(L);n=len(L);segs=[];i=0;pend=0
    def flush_float(upto):
        nonlocal pend
        if upto>=pend and any(L[k].strip() for k in range(pend,upto+1)):
            segs.append(dict(kind='float',start=pend,end=upto,name='',ikind='float',lines=L[pend:upto+1]))
        elif upto>=pend:
            pass
        pend=upto+1
    while i<n:
        info=infos[i];s=L[i].strip()
        if info['depth']!=0:i+=1;continue
        m=REGION.match(L[i])
        if m and info['state']=='code':
            flush_float(i-1)
            segs.append(dict(kind='marker',start=i,end=i,name=m.group(1),ikind='marker',lines=[L[i]],label=m.group(2).strip()))
            pend=i+1;i+=1;continue
        if not s or (s.startswith('//') and not s.startswith('///') and not s.startswith('//!')):i+=1;continue
        if s.startswith('//!'):i+=1;continue
        start=i;j=i;attrs=[]
        while j<n:
            t=L[j].strip()
            if t.startswith('#['):
                k=j;bal=t.count('[')-t.count(']')
                while bal>0 and k+1<n:k+=1;bal+=L[k].count('[')-L[k].count(']')
                attrs.append(' '.join(''.join(L[j:k+1]).split()));j=k+1;continue
            if t.startswith('///') or (t.startswith('//') and not REGION.match(L[j])):j+=1;continue
            if not t:j+=1;continue
            break
        if j>=n:break
        head=L[j].strip()
        k=j
        while k<n:
            if infos[k]['end_depth']==0:
                t=L[k].rstrip()
                if t.endswith(';') or t.endswith('}') or t.endswith('},'):break
            k+=1
        ikind='other';name=''
        for kd,rx in DEF:
            mm=rx.match(head)
            if mm:ikind=kd;name=mm.group(1) if mm.groups() else '';break
        if ikind=='impl':name=re.sub(r'\s+',' ',head.split('{')[0].strip())
        toks=set()
        for q in range(start,k+1):
            for tkn,_ in infos[q]['toks']:toks.add(tkn)
        # leading comments/blank lines since the previous segment belong to this item
        segs.append(dict(kind='item',start=pend,end=k,head=j,ikind=ikind,name=name,attrs=attrs,toks=toks,lines=L[pend:k+1],body=L[start:k+1]))
        pend=k+1;i=k+1
    flush_float(n-1)
    return L,segs,trail
