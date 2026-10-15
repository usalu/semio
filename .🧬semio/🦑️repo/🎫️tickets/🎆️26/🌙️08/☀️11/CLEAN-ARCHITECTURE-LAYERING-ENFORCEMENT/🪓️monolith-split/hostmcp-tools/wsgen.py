import re,sys,os,shutil,collections,json
from wsan import *
STAGE=os.environ.get("WS_STAGE","/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/hm/stage-ws")
CH={c[0]:c for c in CHILDREN}
PATHREWRITES=[]
GR_CHUNKS={"C03","C04","C05","C11"}
SM_CHUNKS={"C02","C11"}
AC_CHUNKS={"C11"}
def cfg_attrs(attrs):
    return tuple(a for a in attrs if a.startswith('#[cfg('))
def line_cfg_context(M,x,it):
    L=M.L;I=M.I
    ctx=list(cfg_attrs(it['attrs']))
    need=I[x]['depth']
    j=x-1
    while j>=it['head'] and (L[j].strip().startswith('#[') or L[j].strip().startswith('//')):
        if L[j].strip().startswith('#[cfg('):ctx.append(' '.join(L[j].split()))
        j-=1
    k=x-1
    while k>=it['head'] and need>0:
        if I[k]['depth']<need and I[k]['state']=='code':
            need=I[k]['depth']
            j=k-1
            while j>=it['head'] and (L[j].strip().startswith('#[') or L[j].strip().startswith('//')):
                if L[j].strip().startswith('#[cfg('):ctx.append(' '.join(L[j].split()))
                j-=1
            inl=re.match(r'^\s*(#\[cfg\([^\n]*?\)\])\s',L[k])
            if inl:ctx.append(inl.group(1))
        k-=1
    return tuple(dict.fromkeys(ctx))
def item_of_line(M):
    arr=[None]*len(M.L)
    for it in M.items:
        for x in range(it['start'],it['end']+1):arr[x]=it
    return arr
def extra_idents(path):
    t=open(path,encoding='utf-8').read().split('\n')
    L=[l+'\n' for l in t]
    I=lex2(L)
    res=[]
    for x,info in enumerate(I):
        for name,col in info['toks']:
            res.append((name,L[x][:col].rstrip(),L[x][col+len(name):].lstrip()))
        for s in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',s):res.append((m.group(1),'',''))
    return res
def analyze(M):
    L=M.L;I=M.I
    arr=item_of_line(M)
    # P-level extra mounted files (any context for widening)
    extra_any=collections.defaultdict(set)
    for f in ["🧪️tests/🔬️quick/🦀️.rs","🧪️tests/🔬️long/🦀️.rs"]:
        for name,pre,post in extra_idents(WS+f):extra_any["P"].add(name)
    p_test_bare=set()
    for f in ["🧪️tests/🔬️quick/🦀️.rs","🧪️tests/🔬️long/🦀️.rs"]:
        for name,pre,post in extra_idents(WS+f):
            if pre.endswith('.') or pre.endswith('::'):continue
            if post.startswith(':') and not post.startswith('::') and name[:1].islower():continue
            if name in M.modnames and not post.startswith('::'):continue
            p_test_bare.add(name)
    c08_test=[]
    for name,pre,post in extra_idents(WS+"🪶️sqlite/🧪️tests/guest-input-slot.rs"):
        extra_any["C08"].add(name)
        if pre.endswith('.') or pre.endswith('::'):continue
        if post.startswith(':') and not post.startswith('::') and name[:1].islower():continue
        c08_test.append(name)
    bare,anyr=ident_refs(M)
    for k,v in extra_any.items():anyr[k]|=v
    # usage cfg per (owner,name)
    usecfg=collections.defaultdict(lambda:collections.defaultdict(list))
    for own in bare:
        for name,lines in bare[own].items():
            for x in lines:
                it=arr[x]
                usecfg[own][name].append(line_cfg_context(M,x,it) if it else ())
    for name in c08_test:
        usecfg["C08"][name].append(("#[cfg(test)]",))
    c08_test_names=set(c08_test)
    return dict(p_test_bare=p_test_bare,arr=arr,bare=bare,anyr=anyr,usecfg=usecfg,c08_test_names=c08_test_names)
def member_scan(M,it):
    """yield (kind,name,line_idx) for private members of inherent impl / struct"""
    L=M.L;I=M.I
    out=[]
    if it['kind']=='impl':
        head=gather_text(L,it).split('{')[0]
        if re.search(r'\bfor\b',head):return out
        for x in range(it['head']+1,it['end']):
            if I[x]['depth']==1 and I[x]['state']=='code':
                m=re.match(r'^(\s*)(pub(?:\([^)]*\))?\s+)?((?:(?:unsafe|async|const)\s+)*)(fn|const|static|type)\s+(\w+)',L[x])
                if m and not m.group(2):out.append(('member',m.group(5),x))
    elif it['kind']=='struct':
        if '{' in L[it['head']] or (it['end']>it['head'] and L[it['head']].rstrip().endswith('{')):
            for x in range(it['head']+1,it['end']):
                if I[x]['depth']==1 and I[x]['state']=='code':
                    m=re.match(r'^(\s*)(pub(?:\([^)]*\))?\s+)?(\w+)\s*:',L[x])
                    if m and not m.group(2):out.append(('field',m.group(3),x))
        elif '(' in L[it['head']]:
            out.append(('tuple',it['name'],it['head']))
    return out
def run(M,A,write=False):
    L=M.L;I=M.I;items=M.items;seg_of=M.seg_of
    bare=A['bare'];anyr=A['anyr'];usecfg=A['usecfg']
    chunk_items=collections.defaultdict(list)
    for it in items:chunk_items[it['owner']].append(it)
    edits=collections.defaultdict(dict) # line -> replacement fn
    widened=collections.defaultdict(list)
    others=lambda own:set().union(*[anyr[o] for o in anyr if o!=own])
    # moved imports: DocumentScope-like use items in children used elsewhere -> move to P
    moved=[]
    for own,its in chunk_items.items():
        if own=="P":continue
        oth=others(own)
        for it in its:
            if it['kind']=='use':
                used_other=[n for n,full in it['uses'] if n in oth]
                if used_other:moved.append((own,it,used_other))
            elif it['name'] and it['vis']=='' and it['name'] in oth:
                edits[it['head']]['item']=it;widened[own].append((it['kind'],it['name'],it['head']))
            elif it['kind']=='macroenum':pass
            for kind,name,x in member_scan(M,it):
                if kind=='tuple':
                    if it['name'] in oth:
                        edits[x]['tuple']=it;widened[own].append(('tuple',it['name'],x))
                elif name in oth:
                    edits[x][kind]=name;widened[own].append((kind,f"{it['name'] or ''}::{name}" if kind!='member' else name,x))
    for own,its in chunk_items.items():
        if own=="P":continue
        types={it['name']:it for it in its if it['kind'] in('struct','enum','type','trait','union') and it['name'] and it['vis']==''}
        changed=True
        while changed:
            changed=False
            for x,e in list(edits.items()):
                if M.seg_of[x][1]!=own:continue
                if 'tuple' in e and 'item' not in e:continue
                sig=L[x];k=x
                if 'item' in e or 'member' in e:
                    while '{' not in sig and ';' not in sig and k+1<len(L):
                        k+=1;sig+=L[k]
                    sig=sig.split('{')[0]
                elif 'field' in e:sig=L[x].split(':',1)[1] if ':' in L[x] else sig
                else:continue
                if re.match(r'^\s*(pub(\([^)]*\))?\s+)?((unsafe |async |const )*)(struct|enum|trait|union)\b',sig) and 'item' in e:continue
                for n,it in list(types.items()):
                    if re.search(r'\b'+n+r'\b',sig) and it['head'] not in edits:
                        edits[it['head']]['item']=it;widened[own].append((it['kind'],n,it['head']));changed=True
                        types.pop(n)
    return chunk_items,edits,widened,moved
if __name__=="__main__":
    M=build();A=analyze(M)
    chunk_items,edits,widened,moved=run(M,A)
    for own in sorted(widened):print(own,len(widened[own]),[f"{k}:{n}" for k,n,_ in widened[own]][:60])
    print("moved",[(o,[n for n in u]) for o,it,u in moved])

TRAITDOC={}
TRAITS=[
 dict(name="GuestRuntime",use='use semio_framework_plugin_host::GuestRuntime as _;',gate=('#[cfg(not(target_arch = "wasm32"))]',),chunks={"C03","C04","C05","C11"},docchunk="C05",methods={"instantiate","execute_turn","compile","drop_instance"}),
 dict(name="SpaceMember",use='use store::SpaceMember as _;',gate=(),chunks={"C02","C11"},docchunk="C11",methods={"undo","redo","close_owned_step","close_owned_terminal_is_empty"}),
 dict(name="ArtifactChannel",use='use super::ArtifactChannel as _;',gate=(),chunks={"C11"},methods={"exchange","activate"}),
]
def ins_vis(line):
    m=re.match(r'^(\s*)(.*)$',line,re.S)
    assert not m.group(2).startswith('pub'),line
    return m.group(1)+'pub(super) '+m.group(2)
def tuple_widen(line):
    m=re.search(r'struct\s+\w+\s*(<[^>]*>)?\s*\(',line)
    assert m,line
    start=m.end();depth=1;i=start
    while i<len(line) and depth>0:
        if line[i]=='(':depth+=1
        elif line[i]==')':depth-=1
        i+=1
    assert depth==0,line
    inner=line[start:i-1]
    parts=[];d=0;cur=''
    for ch in inner:
        if ch in '(<[':d+=1
        if ch in ')>]':d-=1
        if ch==',' and d==0:parts.append(cur);cur=''
        else:cur+=ch
    if cur.strip():parts.append(cur)
    fixed=[]
    for p in parts:
        s=p.lstrip()
        if s.startswith('pub') or s.startswith('#['):fixed.append(p)
        else:fixed.append(p[:len(p)-len(s)]+'pub(super) '+s)
    return line[:start]+','.join(fixed)+line[i-1:]
def marker(l):
    m=re.match(r'^\s*//\s*#(end)?region\s+(.*?)\s*$',l)
    return (bool(m.group(1)),m.group(2)) if m else None
def fix_regions(lines):
    stack=[];pre=[]
    for l in lines:
        mk=marker(l)
        if not mk:continue
        isend,name=mk
        if not isend:stack.append(name)
        else:
            if stack and stack[-1]==name:stack.pop()
            else:pre.append(name)
    head=[f"//#region {n}\n" for n in reversed(pre)]
    tail=[f"//#endregion {n}\n" for n in reversed(stack)]
    return head,tail
def generate(M,A):
    L=list(M.L);I=M.I
    for i,l in enumerate(L):
        if l.startswith("/// 🐎️ The compiled runtime"):TRAITDOC["GuestRuntime"]=L[i:i+3]
        if l.startswith("/// 🌉️ Trait import only"):TRAITDOC["SpaceMember"]=L[i:i+3]
    chunk_items,edits,widened,moved=run(M,A)
    defs=M.defs;bare=A['bare'];usecfg=A['usecfg']
    for own,it,names in moved:
        for n in names:
            defs[n]=[("P",x) if x is it else (o,x) for o,x in defs[n]]
    # apply edits to a copy
    X=list(L)
    for x,e in edits.items():
        if 'item' in e or 'field' in e or 'member' in e:X[x]=ins_vis(X[x])
        if 'tuple' in e:X[x]=tuple_widen(X[x])
    removed=set()
    for own,it,names in moved:
        for x in range(it['start'],it['end']+1):removed.add(x)
    owned=collections.defaultdict(list)
    for k,s,e in M.segs:
        owned[owner_of(k)].append((k,s,e))
    def body_lines(own):
        out=[]
        for k,s,e in owned[own]:
            out+= [X[x] for x in range(s,e) if x not in removed]
        return out
    # needs
    child_ids=[c[0] for c in CHILDREN]
    needs={}
    for c in child_ids:
        names=set()
        for name in list(bare[c].keys())+list(A['c08_test_names'] if c=="C08" else []):
            if name in defs and not any(o==c for o,_ in defs[name]):names.add(name)
        needs[c]=names
    # moved import names defined in P now
    for own,it,ns in moved:
        pass
    def defcfg(name):
        r=set()
        for o,it in defs[name]:
            for a in cfg_attrs(it['attrs']):r.add(a)
        return r
    children_out={}
    summary={}
    for c in child_ids:
        _,folder,mod,emoji,doc=CH[c]
        groups=collections.OrderedDict()
        for name in sorted(needs[c]):
            ctxs=usecfg[c][name]
            sets=[set(t) for t in ctxs] or [set()]
            inter=set.intersection(*sets)
            dc=defcfg(name)
            if len(defs[name])>1:dc=set.intersection(*[set(cfg_attrs(it['attrs'])) for o,it in defs[name]])
            gate=tuple(sorted(inter|dc))
            src=defs[name][0][0]
            srcpath="super" if src=="P" else "super::"+CH[src][2]
            groups.setdefault((srcpath,gate),[]).append(name)
        imports=[]
        for (srcpath,gate) in sorted(groups,key=lambda k:(k[0]!="super",k[0],len(k[1]),k[1])):
            names=groups[(srcpath,gate)]
            for a in gate:imports.append(a+"\n")
            imports.append(("use "+srcpath+"::{"+", ".join(names)+"};\n") if len(names)>1 else ("use "+srcpath+"::"+names[0]+";\n"))
        # traits
        for t in TRAITS:
            if c in t['chunks']:
                hits=[]
                for x,info in enumerate(I):
                    if M.seg_of[x][1]!=c:continue
                    for name,col in info['toks']:
                        if name in t['methods'] and L[x][:col].rstrip().endswith('.'):
                            it=A['arr'][x];hits.append(set(line_cfg_context(M,x,it)) if it else set())
                if hits:
                    gate=set(t['gate'])|set.intersection(*hits)
                    if c==t.get('docchunk'):imports+=TRAITDOC[t['name']]
                    for a in sorted(gate):imports.append(a+"\n")
                    imports.append(t['use']+"\n")
        body=body_lines(c)
        for i,l in enumerate(body):
            m=re.match(r'^(\s*#\[path\s*=\s*")([^"]+)("\]\s*)$',l)
            if m and not m.group(2).startswith("../"):
                body[i]=m.group(1)+"../"+m.group(2)+m.group(3)
                PATHREWRITES.append((c,m.group(2)))
        head,tail=fix_regions(body)
        text=[f"//! {emoji} {doc}\n","\n"]+imports+(["\n"] if imports else [])+head+body+tail
        children_out[c]=text
        summary[c]=(len(imports),len(body))
    return children_out,summary,needs,widened,moved,chunk_items,edits,X,removed,owned

def exports_of(M,X,removed,chunk_items,edits):
    res={}
    for c,_,_,_,_ in CHILDREN:
        rows=[]
        for it in chunk_items[c]:
            if it['kind'] in('impl','use','?','macro'):continue
            vis=it['vis']
            if vis=='' and it['head'] in edits and 'item' in edits[it['head']]:vis='pub(super)'
            if it['kind']=='macroenum':vis='pub'
            if vis:rows.append((vis,set(cfg_attrs(it['attrs']))))
        res[c]=rows
    return res
MCP="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
def outside_idents():
    ids=set()
    for dp,dn,fn in os.walk(MCP):
        dn[:]=[d for d in dn if d not in("node_modules","target","dist","🏠️workspace")]
        for f in fn:
            if f.endswith(".rs"):
                ids|=set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*",open(os.path.join(dp,f),encoding="utf-8",errors="ignore").read()))
    return ids
def api_names(M,ci,edits):
    OUT=outside_idents()
    res={}
    for c,_,_,_,_ in CHILDREN:
        rows=[]
        for it in ci[c]:
            if it['kind'] in('impl','use','?','macro'):continue
            vis=it['vis']
            if it['kind']=='macroenum':vis='pub'
            if vis in('','pub(super)'):continue
            if vis!='pub' and it['name'] not in OUT:continue
            rows.append((vis,tuple(cfg_attrs(it['attrs'])),it['name']))
        cnt=collections.Counter(r[2] for r in rows)
        seen=set();out=[]
        for r in rows:
            if cnt[r[2]]>1:r=(r[0],(),r[2])
            if (r[0],r[1],r[2]) in seen:continue
            seen.add((r[0],r[1],r[2]));out.append(r)
        res[c]=out
    return res
def assemble(M,A):
    co,summ,needs,widened,moved,ci,edits,X,removed,owned=generate(M,A)
    L=M.L
    ex=exports_of(M,X,removed,ci,edits)
    segs={k:(s,e) for k,s,e in M.segs}
    def lines(k):
        s,e=segs[k];return [X[x] for x in range(s,e) if x not in removed]
    P=[]
    p0=lines("P0")
    # insert moved imports after the std::sync use line
    idx=[i for i,l in enumerate(p0) if l.startswith("use std::sync::")]
    assert len(idx)==1
    mv=[]
    for own,it,names in moved:
        mv+= [L[x] for x in range(it['start'],it['end']+1)]
    gi=[i for i,l in enumerate(p0) if l.startswith("/// 🐎️ The compiled runtime")]
    si=[i for i,l in enumerate(p0) if l.startswith("/// 🌉️ Trait import only")]
    assert len(gi)==1 and len(si)==1
    assert p0[gi[0]+4].startswith("use semio_framework_plugin_host::{GuestRuntime as _, ") and p0[gi[0]+3].startswith("#[cfg(not(target_arch")
    assert p0[si[0]+3]=="use store::SpaceMember as _;\n"
    p0[gi[0]+4]=p0[gi[0]+4].replace("{GuestRuntime as _, ","{")
    drop=set(range(gi[0],gi[0]+3))|set(range(si[0],si[0]+4))
    p0=[l for i,l in enumerate(p0) if i not in drop]
    idx=[i for i,l in enumerate(p0) if l.startswith("use std::sync::")]
    p0=p0[:idx[0]+1]+mv+p0[idx[0]+1:]
    P+=p0
    api=api_names(M,ci,edits)
    tests_need=collections.defaultdict(set)
    apinames={n for c in api for v,gt,n in api[c]}
    for name in A['p_test_bare']:
        if name in M.defs and name not in apinames:
            for o,it in M.defs[name]:
                if o!="P" and it['kind']!='use':tests_need[o].add(name)
    def wire(cid):
        _,folder,mod,emoji,doc=CH[cid]
        sets=[set(cfg_attrs(it['attrs'])) for it in ci[cid]]
        common=sorted(set.intersection(*sets)) if sets else []
        g=[a+"\n" for a in common]
        out=g+[f'#[path = "{folder}/🦀️.rs"]\n',f"mod {mod};\n"]
        byvis=collections.OrderedDict()
        for vis,gate,name in api[cid]:
            byvis.setdefault((vis,gate),[]).append(name)
        for (vis,gate),names in byvis.items():
            out+=[a+"\n" for a in gate]
            out.append(f"{vis} use {mod}::{{{', '.join(names)}}};\n" if len(names)>1 else f"{vis} use {mod}::{names[0]};\n")
        if tests_need[cid]:
            tg=collections.OrderedDict()
            for n in sorted(tests_need[cid]):
                gate=set(["#[cfg(test)]"])
                its=[it for o,it in M.defs[n] if o==cid]
                if len(its)>1:gate|=set.intersection(*[set(cfg_attrs(it['attrs'])) for it in its])
                else:gate|=set(cfg_attrs(its[0]['attrs']))
                tg.setdefault(tuple(sorted(gate)),[]).append(n)
            for gate,ns in tg.items():
                out+=[a+"\n" for a in gate]
                out.append(f"use {mod}::{{{', '.join(ns)}}};\n" if len(ns)>1 else f"use {mod}::{ns[0]};\n")
        out.append("\n")
        return out
    for c in ("C01","C02","C03"):P+=wire(c)
    P.append("//#region 🔖️ArtifactChannel\n")
    for c in ("C05","C04","C06","C07","C08"):P+=wire(c)
    P+=lines("P1")
    P.append("//#region 🔖️HeadlessWorkspace\n")
    for c in ("C10","C09","C11","C12"):P+=wire(c)
    P+=lines("P2")
    P+=lines("P3")
    return P,co,ex,summ,widened,needs
def write_stage(stage,P,co):
    shutil.rmtree(stage,ignore_errors=True);os.makedirs(stage)
    open(stage+"/🦀️.rs","w",encoding="utf-8").write("".join(P))
    for cid,folder,mod,emoji,doc in CHILDREN:
        d=stage+"/"+folder;os.makedirs(d)
        open(d+"/🦀️.rs","w",encoding="utf-8").write("".join(co[cid]))
if __name__=="__main__":
    M=build();A=analyze(M)
    P,co,ex,summ,widened,needs=assemble(M,A)
    mode=sys.argv[1] if len(sys.argv)>1 else "stage"
    if mode=="apply":
        import hashlib
        cur=open(SRC,encoding="utf-8").read();orig=open(ORIG,encoding="utf-8").read()
        assert hashlib.sha1(cur.encode()).hexdigest()==hashlib.sha1(orig.encode()).hexdigest(),"workspace file changed since snapshot"
        for cid,folder,mod,emoji,doc in CHILDREN:assert not os.path.exists(WS+folder),folder
        for cid,folder,mod,emoji,doc in CHILDREN:
            os.makedirs(WS+folder);open(WS+folder+"/🦀️.rs","w",encoding="utf-8").write("".join(co[cid]))
        open(SRC,"w",encoding="utf-8").write("".join(P))
        print("applied")
    if mode=="stage":
        write_stage(STAGE,P,co);print("staged",STAGE)
        print({c:sorted(v) for c,v in ex.items()})
