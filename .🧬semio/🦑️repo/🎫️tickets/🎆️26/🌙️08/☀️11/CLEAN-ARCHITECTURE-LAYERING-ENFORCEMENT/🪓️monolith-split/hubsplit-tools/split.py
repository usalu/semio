import sys,os,re,json,collections,hashlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind
from spec import *
TOOLS=os.path.dirname(os.path.abspath(__file__))
REPO='/Users/ueli/Documents/semio/'
HUB=REPO+'🌎️hub/'
BOOT=HUB+'🏗️bootstrap/'
ORIG=TOOLS+'/orig-bootstrap.rs.txt'
src=open(ORIG,encoding='utf-8').read()
assert hashlib.sha256(src.encode()).hexdigest()=='746beedb0ea0b276492799ecda910143411e91134a106d44a384b1da33cb61fe'
GENERATED_MARK='//! OS hub backend'
lines=src.split('\n')
lstart=[0]
for l in lines: lstart.append(lstart[-1]+len(l)+1)
its=items(src)

def parse_tree(toks,i,prefix):
    seg=[]
    while True:
        t=toks[i][1]
        if t=='as':
            return [(prefix+seg,toks[i+1][1])],i+2
        elif toks[i][0]=='id' or t=='*':
            seg.append(t);i+=1
        elif t==':' and toks[i+1][1]==':':
            i+=2
        elif t=='{':
            lv=[];i+=1
            while toks[i][1]!='}':
                l,i=parse_tree(toks,i,prefix+seg);lv+=l
                if toks[i][1]==',':i+=1
            return lv,i+1
        elif t=='as':
            return [(prefix+seg,toks[i+1][1])],i+2
        else:
            break
    return [(prefix+seg,None)],i

rows=[]
for it in its:
    v,k,nm=name_kind(it['toks'])
    ids={t[1] for t in it['toks'] if t[0]=='id'}|it['attr_ids']
    tk=it['toks'];free=set(it['attr_ids']);dot=set()
    for q,t in enumerate(tk):
        if t[0]!='id':continue
        pv=tk[q-1][1] if q>0 and tk[q-1][0]=='p' else ''
        if pv=='.' and not (q>1 and tk[q-2][1]=='.'): dot.add(t[1])
        elif pv==':' and q>1 and tk[q-2][1]==':' : pass
        else: free.add(t[1])
    rows.append(dict(first=it['first_tok_line']+1,hdr=it['hdr_line']+1,end=it['end_line']+1,vis=v,kind=k,name=nm,attrs=it['attrs'],ids=ids,free=free,dot=dot,toks=it['toks']))

leaves=[]
for r in rows:
    if r['kind']!='use':continue
    toks=[t for t in r['toks'] if t[0]!='comment']
    ls,i=parse_tree(toks,1,[])
    cfg=[a for a in r['attrs'] if a.startswith('#[cfg(')]
    for path,alias in ls:
        if path[-1]=='self': path=path[:-1]
        binding=alias if alias else path[-1]
        leaves.append(dict(cfg=cfg,path='::'.join(path),alias=alias,binding=binding,last=path[-1]))
seen=set();ul=[]
for l in leaves:
    key=(tuple(l['cfg']),l['path'],l['alias'])
    if key in seen:continue
    seen.add(key);ul.append(l)
leaves=ul

def render_leaf(l):
    s=''.join(a+'\n' for a in l['cfg'])
    return s+'use '+l['path']+((' as '+l['alias']) if l['alias'] else '')+';'

def render_leaves(ls):
    groups=collections.OrderedDict()
    singles=[]
    for l in ls:
        if l['alias']:
            singles.append(l);continue
        p=l['path']
        if '::' in p:
            parent,last=p.rsplit('::',1)
        else:
            singles.append(l);continue
        groups.setdefault((tuple(l['cfg']),parent),[]).append(last)
    out=[]
    for (cfg,parent),names in groups.items():
        c=''.join(a+'\n' for a in cfg)
        if len(names)==1: out.append(c+'use %s::%s;'%(parent,names[0]))
        else: out.append(c+'use %s::{%s};'%(parent,', '.join(names)))
    for l in singles: out.append(render_leaf(l))
    return '\n'.join(out)

assign={}
for idx,r in enumerate(rows):
    if r['kind'] in('use','crate'):continue
    f=r['first'];mod=None
    for lo,hi,m in RANGES:
        if lo<=f<=hi: mod=m
    if r['name'] in OVERRIDES and mod is not None: mod=OVERRIDES[r['name']]
    if r['name'] in ROOT_KEEP or r['kind']=='mod': mod='ROOT'
    assign[idx]=mod
assert not [i for i,m in assign.items() if m is None]
mods=collections.OrderedDict((m,[]) for m in MODULES)
root_items=[]
for idx,m in assign.items():
    (root_items if m=='ROOT' else mods[m]).append(idx)

STANDALONE=HUB+'🧪️tests/🔬️standalone/🦀️.rs'
def add_virtual():
    ssrc=open(STANDALONE,encoding='utf-8').read()
    for it in items(ssrc):
        v,k,nm=name_kind(it['toks'])
        if k=='use' or not nm: continue
        tk=it['toks'];free=set(it['attr_ids']);dot=set()
        for q,t in enumerate(tk):
            if t[0]!='id':continue
            pv=tk[q-1][1] if q>0 and tk[q-1][0]=='p' else ''
            if pv=='.' and not (q>1 and tk[q-2][1]=='.'): dot.add(t[1])
            elif pv==':' and q>1 and tk[q-2][1]==':' : pass
            else: free.add(t[1])
        rows.append(dict(first=-1,hdr=-1,end=-1,vis=v,kind=k,name=nm,attrs=it['attrs'],ids={t[1] for t in tk if t[0]=='id'}|it['attr_ids'],free=free,dot=dot,toks=tk,virtual=True,ssrc=ssrc,sit=it))
        mods['access_grants'].append(len(rows)-1)
add_virtual()

def file_ids3(path):
    tk=lex(open(path,encoding='utf-8').read())
    allids=set();free=set();dot=set()
    for q,t in enumerate(tk):
        if t[0]!='id':continue
        allids.add(t[1])
        pv=tk[q-1][1] if q>0 and tk[q-1][0]=='p' else ''
        if pv=='.' and not (q>1 and tk[q-2][1]=='.'): dot.add(t[1])
        elif pv==':' and q>1 and tk[q-2][1]==':': pass
        else: free.add(t[1])
    return allids,free,dot
cmd_files=['🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs','🔐️auth/📤️command/🦀️.rs']
test_files=['🧪️tests/🔬️bin-unit/🦀️.rs','🧪️tests/🤝️two-client-document/🦀️.rs','🧪️tests/🗂️artifact-root/🦀️.rs','🧪️tests/🔬️bin-unit/🪶️count-lease/🦀️.rs','🧪️tests/🎲️hostile-generative/🦀️.rs']
def union3(files):
    a=set();f=set();d=set()
    for p in files:
        x,y,z=file_ids3(HUB+p);a|=x;f|=y;d|=z
    return a,f,d
cmd3=union3(cmd_files)
test3=union3(test_files)
t3=tuple(cmd3[i]|test3[i] for i in range(3))
tids=t3[0]
root3=[set(),set(),set()]
for idx in root_items:
    root3[0]|=rows[idx]['ids'];root3[1]|=rows[idx]['free'];root3[2]|=rows[idx]['dot']
rootids=root3[0]

modids={m:set() for m in mods}
modfree={m:set() for m in mods}
moddot={m:set() for m in mods}
for m,idxs in mods.items():
    for i in idxs:
        modids[m]|=rows[i]['ids'];modfree[m]|=rows[i]['free'];moddot[m]|=rows[i]['dot']

TRAIT_SRC=json.load(open(TOOLS+'/traits.json'))
TRAIT_SRC.update({'PayloadStorage':[['🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs',5799]],'SessionStore':[['🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs',376]]})
def trait_methods_of_tokens(toks):
    out=set();depth=0;started=False
    for k,t in enumerate(toks):
        x=t[1]
        if t[0]=='p':
            if x in '{([': 
                depth+=1
                if x=='{' and not started: started=True;base=depth
            elif x in '})]': depth-=1
        if started and t[0]=='id' and x=='fn' and depth==base and toks[k+1][0]=='id': out.add(toks[k+1][1])
    return out
EXT_TRAIT_METHODS={}
for name,locs in TRAIT_SRC.items():
    f,ln=locs[0]
    if f.startswith('.🧬semio'):continue
    try: text=open(REPO+f,encoding='utf-8').read()
    except Exception: continue
    tk=lex(text)
    for k,t in enumerate(tk):
        if t[2]==ln-1 and t[0]=='id' and t[1]=='trait' and tk[k+1][1]==name:
            end=k
            d=0;st=False
            j=k
            while True:
                if tk[j][0]=='p':
                    if tk[j][1]=='{': d+=1;st=True
                    elif tk[j][1]=='}':
                        d-=1
                        if st and d==0:break
                j+=1
            EXT_TRAIT_METHODS[name]=trait_methods_of_tokens(tk[k:j+1])
            break
EXT_TRAIT_METHODS['IntoResponse']={'into_response'}
EXT_TRAIT_METHODS['SinkExt']={'send','feed','flush','close','send_all','with','sink_map_err','sink_err_into','buffer','fanout','with_flat_map'}
EXT_TRAIT_METHODS['StreamExt']={'next','map','filter','filter_map','split','for_each','collect','fuse','take','skip','then','chain','zip','boxed','peekable','by_ref','take_while','skip_while','flat_map','fold','forward','into_future','enumerate','inspect','select_next_some','for_each_concurrent','buffered','buffer_unordered','scan','flatten','chunks','count','left_future','right_future','poll_next_unpin','next_if'}

# local traits and their methods
LOCAL_TRAITS={}
for m,idxs in mods.items():
    for i in idxs:
        r=rows[i]
        if r['kind']=='trait': LOCAL_TRAITS[r['name']]=(m,trait_methods_of_tokens(r['toks']))

def leaf_trait_name(l):
    return l['last'] if l['alias']=='_' else l['binding']
def keep_leaf(l,free,dot):
    if l['binding'] in free and l['alias']!='_': return True
    tn=leaf_trait_name(l)
    ms=EXT_TRAIT_METHODS.get(tn)
    if ms and (ms&dot): return True
    return False

# ---- members
def skip_attrs(toks,i):
    while i<len(toks) and toks[i][0]=='p' and toks[i][1]=='#':
        d=0;i+=1
        if toks[i][1]=='!':i+=1
        while True:
            if toks[i][1]=='[':d+=1
            elif toks[i][1]==']':
                d-=1
                if d==0:break
            i+=1
        i+=1
    return i

def split_members(toks,open_i,kind):
    members=[];d=0;i=open_i
    # i at '{' or '('
    closer={'{':'}','(':')'}[toks[i][1]]
    i+=1
    base=1
    start=i;ang=0
    cur=[]
    while True:
        t=toks[i]
        x=t[1]
        if t[0]=='p':
            if x in '{([':
                base+=1
            elif x in '})]':
                base-=1
                if base==0:
                    if cur or i>start:
                        members.append((start,i))
                    break
                if kind=='impl' and base==1 and x=='}':
                    nxt=toks[i+1][1]
                    if nxt!=';':
                        members.append((start,i));start=i+1
            elif x==';' and base==1:
                members.append((start,i));start=i+1
            elif kind=='fields':
                if x=='<' : ang+=1
                elif x=='>' and not (toks[i-1][1]=='-'):ang=max(0,ang-1)
                elif x==',' and base==1 and ang==0:
                    members.append((start,i));start=i+1
        i+=1
    return members

def member_info(toks,a,b,kind):
    i=skip_attrs(toks,a)
    while i<b and toks[i][0]=='comment':i+=1
    if i>=b: return None
    ins=toks[i][4];hasvis=toks[i][1]=='pub'
    j=i
    if hasvis:
        j+=1
        if toks[j][1]=='(':
            while toks[j][1]!=')':j+=1
            j+=1
    if kind=='fields':
        if toks[j][0]=='id' and j+1<b and toks[j+1][1]==':':
            return dict(name=toks[j][1],ins=ins,hasvis=hasvis,sig=set())
        return dict(name=None,ins=ins,hasvis=hasvis,sig={t[1] for t in toks[j:b] if t[0]=='id'})
    # impl member
    k=j
    while toks[k][1] in('async','unsafe','extern','default') or toks[k][0]=='str' or (toks[k][1]=='const' and toks[k+1][1] in('fn','async','unsafe','extern')): k+=1
    kw=toks[k][1]
    if kw in('fn','const','type'):
        name=toks[k+1][1]
        # signature ids up to first '{' or ';' at relative depth0
        d=0;sig=set()
        for t in toks[k:b+1]:
            if t[0]=='p':
                if t[1]=='{' and d==0:break
                if t[1] in '([{':d+=1
                elif t[1] in ')]}':d-=1
                elif t[1]==';' and d==0:break
            if t[0]=='id':sig.add(t[1])
        return dict(name=name,ins=ins,hasvis=hasvis,sig=sig)
    return None

def row_members(r):
    toks=r['toks']
    # locate header keyword
    i=0
    if toks[i][1]=='pub':
        i+=1
        if toks[i][1]=='(':
            while toks[i][1]!=')':i+=1
            i+=1
    while toks[i][1] in('async','unsafe','extern','default') or toks[i][0]=='str': i+=1
    kw=toks[i][1]
    if kw=='struct' or kw=='union':
        j=i+2;ang=0
        while True:
            x=toks[j][1]
            if x=='<':ang+=1
            elif x=='>' and toks[j-1][1]!='-':ang-=1
            elif ang==0 and x in '{(;' : break
            j+=1
        if toks[j][1]==';':return 'fields',[]
        ms=split_members(toks,j,'fields')
        out=[]
        for a,b in ms:
            mi=member_info(toks,a,b,'fields')
            if mi:out.append(mi)
        return ('tuple' if toks[j][1]=='(' else 'fields'),out
    if kw=='impl':
        j=i+1;ang=0;isfor=False
        while True:
            x=toks[j][1]
            if x=='<':ang+=1
            elif x=='>' and toks[j-1][1]!='-':ang-=1
            elif ang==0 and x=='for' and toks[j][0]=='id': isfor=True
            elif ang==0 and x=='{': break
            j+=1
        if isfor:return 'traitimpl',[]
        ms=split_members(toks,j,'impl')
        out=[]
        for a,b in ms:
            mi=member_info(toks,a,b,'impl')
            if mi:out.append(mi)
        return 'impl',out
    return None,[]

TYPEKINDS=('struct','enum','trait','type','union')
def item_sig_ids(r):
    toks=r['toks']
    if r['kind']=='fn':
        d=0;s=set()
        for t in toks:
            if t[0]=='p':
                if t[1]=='{' and d==0:break
                if t[1] in '([{':d+=1
                elif t[1] in ')]}':d-=1
            if t[0]=='id':s.add(t[1])
        return s
    return set(r['ids'])

def analyze():
    # returns per-module: pubnames(set), member_inserts: {row index: [abs offsets]}
    res={}
    for m,idxs in mods.items():
        ext=set(tids)|rootids
        extfree=set(t3[1])|root3[1]
        extdot=set(t3[2])|root3[2]
        for o in mods:
            if o!=m:
                ext|=modids[o];extfree|=modfree[o];extdot|=moddot[o]
        byname=collections.defaultdict(list)
        for i in idxs:
            if rows[i]['name']: byname[rows[i]['name']].append(i)
        pub=set()
        for i in idxs:
            r=rows[i]
            if r['name'] and r['name'] in extfree and r['kind'] in('fn','struct','enum','trait','type','const','static','union'): pub.add(i)
        # local traits needed because their methods are called elsewhere
        for i in idxs:
            r=rows[i]
            if r['kind']=='trait':
                meths=LOCAL_TRAITS[r['name']][1]
                if meths&extdot: pub.add(i)
        member_ins={}
        changed=True
        pubmember_sigs={}
        while changed:
            changed=False
            for i in idxs:
                r=rows[i]
                kind,ms=row_members(r)
                if kind is None: continue
                if kind in('fields','tuple'):
                    if i not in pub: continue
                    ins=[]
                    for mi in ms:
                        if mi['hasvis']:continue
                        if kind=='tuple' or (mi['name'] in ext):
                            ins.append(mi['ins']);pubmember_sigs.setdefault(i,set()).update(mi['sig'])
                            if mi['name'] is None: pass
                    member_ins[i]=ins
                elif kind=='impl':
                    ins=[]
                    # type of impl must be nameable; widen methods used elsewhere
                    for mi in ms:
                        if mi['hasvis']:continue
                        if mi['name'] in ext:
                            ins.append(mi['ins']);pubmember_sigs.setdefault(i,set()).update(mi['sig'])
                    member_ins[i]=ins
            # signature closure
            need=set()
            for i in list(pub)+list(pubmember_sigs):
                sig=item_sig_ids(rows[i]) if i in pub else set()
                sig|=pubmember_sigs.get(i,set())
                for n in sig:
                    for q in byname.get(n,[]):
                        if rows[q]['kind'] in TYPEKINDS and q not in pub:
                            need.add(q)
            if need:
                pub|=need;changed=True
        res[m]=(pub,member_ins)
    return res

ANALYSIS=analyze()

def item_text(i,pub,member_ins):
    r=rows[i]
    a=lstart[r['first']-1]
    b=lstart[r['end']]-1
    text=src[a:b]
    ops=[]
    if pub and r['vis']=='' and r['kind'] in('fn','struct','enum','trait','type','const','static','union'):
        ops.append(r['toks'][0][4])
    for o in member_ins: ops.append(o)
    for o in sorted(ops,reverse=True):
        h=o-a
        text=text[:h]+'pub(crate) '+text[h:]
    text=text.replace('include!("../🧪️tests/🔬️standalone/🦀️.rs");','include!("../../🧪️tests/🔬️standalone/🦀️.rs");')
    out=[]
    for ln in text.split('\n'):
        if re.match(r'^\s*//#(end)?region',ln):continue
        out.append(ln)
    while out and out[0].strip()=='':out.pop(0)
    while out and out[-1].strip()=='':out.pop()
    return '\n'.join(out)

defs={m:collections.OrderedDict() for m in mods}
for m,idxs in mods.items():
    for i in idxs:
        r=rows[i]
        if r['name'] and r['kind'] not in('impl','use','mod','include','macro','crate'):
            defs[m].setdefault(r['name'],[]).append(i)

def cfg_of(idxs):
    cfg=[]
    for i in idxs:
        for a in rows[i]['attrs']:
            if a.startswith('#[cfg(') and a not in cfg: cfg.append(a)
    return cfg

PRUNE_PATH=TOOLS+'/prune.json'
def load_prune():
    try: return json.load(open(PRUNE_PATH))
    except Exception: return {}
def apply_prune(ls,key):
    pr=load_prune().get(key,{})
    out=[]
    for l in ls:
        b=l['binding']
        if b in pr.get('drop',[]): continue
        if b in pr.get('test',[]):
            l=dict(l)
            if any('test' in a for a in l['cfg']): continue
            if True:
                l['cfg']=['#[cfg(test)]'] if not l['cfg'] else ['#[cfg(all(test,%s))]'%l['cfg'][0][len('#[cfg('):-2]]
        out.append(l)
    return out

def gen_module(m):
    folder,emoji,desc=MODULES[m]
    pub,member_ins=ANALYSIS[m]
    parts=[item_text(i,i in pub,member_ins.get(i,[])) for i in mods[m] if not rows[i].get('virtual')]
    body='\n\n'.join(parts)
    ids=modfree[m]
    cross=[];seenc=set()
    for o in mods:
        if o==m:continue
        for name,idxs in defs[o].items():
            need=name in ids
            if not need and name in LOCAL_TRAITS and LOCAL_TRAITS[name][0]==o and (LOCAL_TRAITS[name][1]&moddot[m]): need=True
            if need and rows[idxs[0]]['kind'] in('fn','struct','enum','trait','type','const','static','union'):
                if not any(i in ANALYSIS[o][0] for i in idxs):
                    # name only appears as a local variable collision; item stays private -> no import
                    if name not in ids or True: pass
                cfg=cfg_of(idxs)
                if any(i in ANALYSIS[o][0] for i in idxs):
                    cross.append(dict(cfg=cfg,path='crate::'+o+'::'+name,alias=None,binding=name,last=name))
    kept=[l for l in leaves if keep_leaf(l,modfree[m],moddot[m])]
    ls=apply_prune(kept+cross,m)
    head='//! %s `%s` — %s.\n//!\n//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.\n\n'%(emoji,m,desc)
    return head+render_leaves(ls)+'\n\n'+body+'\n'

def gen_root():
    hdr=[]
    i=0
    while lines[i].startswith('//!'):
        hdr.append(lines[i]);i+=1
    ntf=root3[1]|cmd3[1];ntd=root3[2]|cmd3[2]
    nt=ntf
    kept_plain=[l for l in leaves if keep_leaf(l,ntf,ntd)]
    kept_test=[l for l in leaves if keep_leaf(l,t3[1],t3[2]) and not keep_leaf(l,ntf,ntd)]
    kept_plain=apply_prune(kept_plain,'ROOT');kept_test=apply_prune(kept_test,'ROOT')
    root=['\n'.join(hdr),'','#[cfg(test)]\nextern crate directory as semio_framework_os_kernel;','']
    def with_test(l):
        c=[a for a in l['cfg']]
        if '#[cfg(test)]' not in c: c=['#[cfg(test)]']+c if not c else c
        return dict(l,cfg=c)
    def test_leaf(l):
        if any('test' in a for a in l['cfg']): return l
        if l['cfg']:
            inner=l['cfg'][0][len('#['):-1]
            return dict(l,cfg=['#[cfg(all(test,%s))]'%inner[len('cfg('):-1]]) if len(l['cfg'])==1 else l
        return dict(l,cfg=['#[cfg(test)]'])
    root.append(render_leaves(kept_plain))
    root.append(render_leaves([test_leaf(l) for l in kept_test]))
    root.append('')
    for m,(folder,emoji,desc) in MODULES.items():
        root.append('#[path = "%s/🦀️.rs"]\nmod %s;'%(folder,m))
    root.append('')
    for m in MODULES:
        names={n for n,idxs in defs[m].items() if any(i in ANALYSIS[m][0] for i in idxs)}
        trait_hit=any(tm==m and (ms&ntd) for t,(tm,ms) in LOCAL_TRAITS.items())
        trait_hit_t=any(tm==m and (ms&t3[2]) for t,(tm,ms) in LOCAL_TRAITS.items())
        names|={t for t,(tm,ms) in LOCAL_TRAITS.items() if tm==m}
        if names&nt or trait_hit: root.append('use %s::*;'%m)
        elif names&t3[1] or trait_hit_t: root.append('#[cfg(test)]\nuse %s::*;'%m)
    root.append('')
    for idx in root_items:
        if rows[idx]['kind']!='mod': root.append(item_text(idx,False,[])+'\n')
    for idx in root_items:
        if rows[idx]['kind']=='mod': root.append(item_text(idx,False,[])+'\n')
    return '\n'.join(root)

def build():
    out={m:gen_module(m) for m in mods}
    out['ROOT']=gen_root()
    return out

def path_of(m):
    return BOOT+'🦀️.rs' if m=='ROOT' else BOOT+MODULES[m][0]+'/🦀️.rs'

if __name__=='__main__':
    out=build()
    stage=TOOLS+'/stage'
    os.makedirs(stage,exist_ok=True)
    for m,t in out.items():
        p=os.path.join(stage,'ROOT.rs' if m=='ROOT' else MODULES[m][0]+'.rs')
        open(p,'w',encoding='utf-8').write(t)
        print(m,t.count('\n'))
