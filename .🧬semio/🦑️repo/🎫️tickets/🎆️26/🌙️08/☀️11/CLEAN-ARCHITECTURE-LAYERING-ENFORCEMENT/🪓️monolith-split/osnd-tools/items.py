"""🔎 Top-level item table of one Rust file: kind, name, line range, region, bytes, and referenced sibling item names."""
import re,sys,collections
from rlex2 import lex2
KW=r'(?:pub(?:\([^)]*\))?\s+)?'
DEF=[('struct',re.compile(r'^'+KW+r'struct\s+(\w+)')),('enum',re.compile(r'^'+KW+r'enum\s+(\w+)')),('union',re.compile(r'^'+KW+r'union\s+(\w+)')),
 ('fn',re.compile(r'^'+KW+r'(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+(\w+)')),('const',re.compile(r'^'+KW+r'const\s+(\w+)')),('static',re.compile(r'^'+KW+r'static\s+(?:mut\s+)?(\w+)')),
 ('type',re.compile(r'^'+KW+r'type\s+(\w+)')),('trait',re.compile(r'^'+KW+r'(?:unsafe\s+)?trait\s+(\w+)')),('mod',re.compile(r'^'+KW+r'mod\s+(\w+)')),('macro',re.compile(r'^macro_rules!\s*(\w+)')),
 ('impl',re.compile(r'^(?:unsafe\s+)?impl\b')),('use',re.compile(r'^'+KW+r'use\b')),('extern',re.compile(r'^extern\s+crate\b'))]
def parse(path):
    L=open(path,encoding='utf-8').read().split('\n')
    if L and L[-1]=='':L=L[:-1]
    L=[l+'\n' for l in L]
    infos=lex2(L)
    items=[];i=0;n=len(L);region=[];
    while i<n:
        info=infos[i];s=L[i].strip()
        if info['depth']!=0 or not s:i+=1;continue
        m=re.match(r'^//\s*#region\s*(.*)',s)
        if m and info['depth']==0:region.append(m.group(1).strip());i+=1;continue
        if re.match(r'^//\s*#endregion',s):
            if region:region.pop()
            i+=1;continue
        if s.startswith('//'):i+=1;continue
        start=i;j=i;attrs=[]
        while j<n:
            t=L[j].strip()
            if t.startswith('#['):
                k=j;bal=t.count('[')-t.count(']')
                while bal>0 and k+1<n:k+=1;bal+=L[k].count('[')-L[k].count(']')
                attrs.append(' '.join(''.join(L[j:k+1]).split()));j=k+1;continue
            if t.startswith('///') or t.startswith('//!') :j+=1;continue
            break
        if j>=n:break
        head=L[j].strip()
        # end of item
        k=j
        while k<n:
            if infos[k]['end_depth']==0:
                t=L[k].rstrip()
                if t.endswith(';') or t.endswith('}') or t.endswith('},') :break
            k+=1
        kind='other';name=''
        for kd,rx in DEF:
            mm=rx.match(head)
            if mm:
                kind=kd;name=mm.group(1) if mm.groups() else '';break
        if kind=='impl':name=re.sub(r'\s+',' ',head.split('{')[0].strip())
        toks=set()
        for q in range(start,k+1):
            for tkn,_ in infos[q]['toks']:toks.add(tkn)
        items.append(dict(start=start,head=j,end=k,kind=kind,name=name,attrs=attrs,region='/'.join(region),bytes=sum(len(l.encode()) for l in L[start:k+1]),lines=k-start+1,toks=toks,text=''.join(L[start:k+1])))
        i=k+1
    return L,items
if __name__=='__main__':
    path=sys.argv[1];L,items=parse(path)
    names={it['name']:it for it in items if it['kind'] in('struct','enum','fn','const','static','type','trait','mod','macro','union')}
    for idx,it in enumerate(items):
        refs=sorted(n for n in names if n in it['toks'] and n!=it['name'])
        print(f"{it['start']+1}-{it['end']+1}\t{it['kind']}\t{it['name'][:60]}\t{it['region']}\t{it['lines']}L/{it['bytes']}B\t{','.join(it['attrs'])[:40]}")
