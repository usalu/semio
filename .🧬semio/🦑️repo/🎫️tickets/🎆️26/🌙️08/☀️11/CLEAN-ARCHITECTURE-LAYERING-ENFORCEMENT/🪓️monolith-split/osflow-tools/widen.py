"""📣 Cross-crate visibility widening: every non-`pub` definition used by a file of ANOTHER crate becomes `pub` (name-based, conservative)."""
import os,re,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
def load(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];return L,lex2(L)
def uses(p,cache={}):
    if p in cache:return cache[p]
    L,I=load(p);f=set();m=set();a=set()
    for x,info in enumerate(I):
        line=L[x]
        for name,col in info['toks']:
            pre=line[:col].rstrip();post=line[col+len(name):].lstrip()
            a.add(name)
            if pre.endswith('.') and not pre.endswith('..'):f.add(name);m.add(name)
            if pre.endswith('::'):m.add(name)
            if post.startswith(':') and not post.startswith('::'):f.add(name)
            if post.startswith('(') or post.startswith('::<'):m.add(name)
        for s in info['strs']:
            for mm in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',s):a.add(mm.group(1))
    cache[p]=(f,m,a);return cache[p]
VISRX=re.compile(r'^(\s*)(?:pub\((?:super|crate|in [\w:]+)\)\s+)?')
def defs(p):
    """(line_index, indent, kind, name) of widenable non-pub definitions"""
    L,I=load(p);out=[]
    for it in top_items(L,I):
        classify(it)
        h=it['head']
        if it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod') and it.get('name') and it['vis'] in('','pub(super)','pub(crate)') or (it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod') and it.get('name') and it['vis'].startswith('pub(in')):
            out.append((h,0,'item',it['name'],None))
        if it['kind']=='struct':
            for x in range(h+1,it['end']+1):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^(\s*)(pub\((?:super|crate|in [\w:]+)\)\s+|pub\s+)?(\w+)\s*:',L[x])
                    if m and (not m.group(2) or m.group(2).startswith('pub(')):out.append((x,len(m.group(1)),'field',m.group(3),it.get('name')))
            tm=re.search(r'struct\s+(\w+)(<[^{(]*?>)?\(',L[h])
            if tm:out.append((h,0,'tuple',tm.group(1),None))
        if it['kind']=='impl':
            head=L[h]
            if re.search(r'\bfor\b',head):continue
            om=re.match(r'^\s*impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)',head)
            impl_owner=om.group(1) if om else None
            for x in range(h+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^(\s*)(pub\((?:super|crate|in [\w:]+)\)\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const)\s+(\w+)',L[x])
                    if m:out.append((x,len(m.group(1)),'method',m.group(5),impl_owner))
    return L,out
STOP=set("""get take len is_empty push pop last_mut get_mut step close_step terminal_is_empty advance new default fmt from into clone drop eq iter next insert remove contains map keys values first last extend clear as_ref as_mut unwrap text number integer boolean bytes ok object value kind id name state source input output items depth phase target handle current action page stage cursor""".split())
def allowed_pairs(deps):
    """deps: crate id -> set of direct internal deps; returns set of (provider, consumer) with consumer depending transitively on provider"""
    closure={}
    def reach(c,seen):
        for d in deps.get(c,()):
            if d not in seen:seen.add(d);reach(d,seen)
        return seen
    out=set()
    for c in deps:
        for d in reach(c,set()):out.add((d,c))
    return out
def widen(owner,dry=False,skip=(),deps=None,providers=None):
    report={}
    files=list(owner)
    pairs=allowed_pairs(deps) if deps else None
    # global multiplicity of method/field names defined anywhere
    mult={}
    alldefs={}
    for p in files:
        L,dl=defs(p);alldefs[p]=(L,dl)
        L2,I2=load(p)
    for p in files:
        for idx,ind,kind,name,own in alldefs[p][1]:
            mult.setdefault((kind,name),set()).add(p)
    for p in files:
        if providers is not None and p not in providers:continue
        cid=owner[p]
        L,dl=alldefs[p]
        changed=False
        for idx,ind,kind,name,own in dl:
            if name in skip or name in STOP and kind in('method','field'):continue
            hit=None
            for q in files:
                if owner[q]==cid:continue
                if pairs is not None and (cid,owner[q]) not in pairs:continue
                f,m,a=uses(q)
                if kind=='item':ok=name in a
                elif kind=='tuple':ok=name in a
                elif kind=='field':ok=name in f and (own in a or len(mult[(kind,name)])==1)
                else:ok=name in m and (own in a or len(mult[(kind,name)])==1)
                if ok:hit=q;break
            if not hit:continue
            l=L[idx]
            if kind=='tuple':
                tm=re.search(r'(struct\s+\w+(?:<[^{(]*?>)?\()(.*)(\)[^()]*;\s*)$',l)
                if not tm:continue
                parts=[];depth=0;cur=''
                for ch in tm.group(2):
                    if ch in '<([{':depth+=1
                    if ch in '>)]}':depth-=1
                    if ch==',' and depth==0:parts.append(cur);cur=''
                    else:cur+=ch
                if cur.strip():parts.append(cur)
                np=[x if x.strip().startswith('pub ') else (' '*(len(x)-len(x.lstrip()))+'pub '+re.sub(r'^pub\([^)]*\)\s+','',x.lstrip())) for x in parts]
                nl=l[:tm.start(2)]+','.join(np)+l[tm.end(2):]
                if nl!=l:L[idx]=nl;changed=True;report.setdefault(cid,[]).append((os.path.basename(os.path.dirname(p)),name+'.0'))
                continue
            m=re.match(r'^(\s*)(pub\((?:super|crate|in [\w:]+)\)\s+)?',l)
            nl=m.group(1)+'pub '+l[m.end():]
            if nl!=l:L[idx]=nl;changed=True;report.setdefault(cid,[]).append((os.path.basename(os.path.dirname(p)),name))
        if changed and not dry:open(p,'w',encoding='utf-8').write(''.join(L))
    return report
if __name__=="__main__":
    pass
