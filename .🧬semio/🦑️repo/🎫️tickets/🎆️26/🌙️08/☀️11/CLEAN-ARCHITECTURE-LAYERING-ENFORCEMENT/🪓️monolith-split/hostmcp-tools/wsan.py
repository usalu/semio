import re,sys,os,collections
from wscommon import *
VIS=r'(?:pub(?:\([^)]*\))?\s+)?'
KW=r'(?:(?:unsafe|async|const|extern\s+"[^"]*")\s+)*'
def parse_use(text):
    """text: 'use ...;' (attrs stripped). returns (names:list[(name,full)], anon:list[full], globs:list[path])"""
    body=text.strip()
    body=re.sub(r'^(pub(\([^)]*\))?\s+)?use\s+','',body).rstrip(';').strip()
    names=[];anon=[];globs=[]
    def walk(prefix,s):
        s=s.strip()
        # split top-level commas
        parts=[];depth=0;cur=''
        for ch in s:
            if ch=='{':depth+=1
            if ch=='}':depth-=1
            if ch==',' and depth==0:parts.append(cur);cur=''
            else:cur+=ch
        if cur.strip():parts.append(cur)
        for p in parts:
            p=p.strip()
            if not p:continue
            m=re.match(r'^(.*?)::\{(.*)\}$',p,re.S)
            if m:
                walk(prefix+m.group(1).strip()+'::',m.group(2));continue
            if p.startswith('{') and p.endswith('}'):
                walk(prefix,p[1:-1]);continue
            if p.endswith('::*') or p=='*':
                globs.append(prefix+p[:-3] if p!='*' else prefix.rstrip(':'));continue
            m=re.match(r'^(.*?)\s+as\s+(\w+)$',p)
            if m:
                full=prefix+m.group(1).strip()
                if m.group(2)=='_':anon.append(full)
                else:names.append((m.group(2),full))
                continue
            if p=='self':
                names.append((prefix.rstrip(':').split('::')[-1],prefix.rstrip(':')));continue
            full=prefix+p
            names.append((p.split('::')[-1],full))
    walk('',body)
    return names,anon,globs
def top_items(L,I):
    items=[];n=len(L);i=0
    while i<n:
        if I[i]['depth']!=0 or I[i]['state']!='code':i+=1;continue
        s=L[i].strip()
        if not s or s.startswith('//'):i+=1;continue
        start=i;j=i;attrs=[]
        while j<n:
            t=L[j].strip()
            if t.startswith('#['):
                k=j;txt=L[k];bal=txt.count('[')-txt.count(']')
                while bal>0 and k+1<n:k+=1;txt+=L[k];bal+=L[k].count('[')-L[k].count(']')
                attrs.append(' '.join(txt.split()));j=k+1;continue
            if t.startswith('//'):j+=1;continue
            break
        if j>=n:break
        k=j
        while k<n:
            if I[k]['end_depth']==0 and I[k]['state']=='code':
                t=L[k].rstrip()
                if t.endswith(';') or t.endswith('}'):break
            k+=1
        items.append(dict(start=start,head=j,end=k,attrs=attrs,text=L[j].strip()))
        i=k+1
    return items
def classify(it):
    h=it['text']
    m=re.match(r'^(pub(?:\(([^)]*)\))?\s+)?'+KW+r'(fn|struct|enum|const|static|type|trait|mod|union)\s+(r#)?(\w+)',h)
    if m:
        vis='' if not m.group(1) else (('pub('+m.group(2)+')') if m.group(2) else 'pub')
        it.update(kind=m.group(3),name=m.group(5),vis=vis);return
    if re.match(r'^(pub(\([^)]*\))?\s+)?use\s',h):
        it.update(kind='use',name=None,vis='pub' if h.startswith('pub') else '');return
    if re.match(r'^(unsafe\s+)?impl\b',h):
        it.update(kind='impl',name=None,vis='');return
    m=re.match(r'^(\w+)!\s*\{?',h)
    if m:
        it.update(kind='macro',name=None,vis='');return
    it.update(kind='?',name=None,vis='');
def gather_text(L,it):
    return ''.join(L[it['head']:it['end']+1])

class Model: pass
def build():
    L,I=load()
    segs=segments(L)
    seg_of=[None]*len(L)
    for k,s,e in segs:
        for x in range(s,e):seg_of[x]=(k,owner_of(k))
    M=Model();M.L=L;M.I=I;M.segs=segs;M.seg_of=seg_of
    items=top_items(L,I)
    for it in items:
        classify(it)
        ks={seg_of[x][0] for x in range(it['start'],it['end']+1)}
        assert len(ks)==1,(it['text'][:60],ks)
        it['seg']=ks.pop();it['owner']=owner_of(it['seg'])
        if it['kind']=='macro':
            m=re.search(r'enum\s+(\w+)',gather_text(L,it))
            if m:it['name']=m.group(1);it['vis']='pub';it['kind']='macroenum'
    M.items=items
    # definitions
    defs={} # name -> list of (owner,item)
    for it in items:
        if it['kind']=='use':
            names,anon,globs=parse_use(gather_text(L,it))
            it['uses']=names;it['anon']=anon;it['globs']=globs
            for n,full in names:defs.setdefault(n,[]).append((it['owner'],it))
        elif it['name']:
            defs.setdefault(it['name'],[]).append((it['owner'],it))
    M.defs=defs
    M.modnames={n for n,v in defs.items() if any(it['kind']=='mod' for o,it in v)}
    return M
def toks_of_line(M,x):
    return M.I[x]['toks']
def ident_refs(M):
    """returns per owner: bare: set names used as path-first idents; anyc: Counter of any ident; and per (owner,name): list of line idx for bare"""
    L=M.L;I=M.I
    bare=collections.defaultdict(lambda:collections.defaultdict(list))
    anyr=collections.defaultdict(set)
    for x,info in enumerate(I):
        own=M.seg_of[x][1]
        if info['state'] in('blk',):continue
        line=L[x]
        for name,col in info['toks']:
            anyr[own].add(name)
            pre=line[:col].rstrip()
            post=line[col+len(name):].lstrip()
            if pre.endswith('.') and not pre.endswith('..'):continue
            if pre.endswith('::'):continue
            if post.startswith(':') and not post.startswith('::') and name[:1].islower() and name not in ('self',):continue
            if name in M.modnames and not post.startswith('::'):continue
            bare[own][name].append(x)
        for s in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',s):
                anyr[own].add(m.group(1));bare[own][m.group(1)].append(x)
    return bare,anyr
