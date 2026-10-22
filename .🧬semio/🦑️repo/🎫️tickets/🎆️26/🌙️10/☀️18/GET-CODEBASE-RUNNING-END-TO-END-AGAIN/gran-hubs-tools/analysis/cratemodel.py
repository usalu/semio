"""🏛️ Crate model: module nodes from a lib root (`mod`, `#[path]`, inline mods), definitions, `use` edges and public-name resolution to the defining file. `Model(libfile)`."""
import os,re
from rs import *
ROOT='/Users/ueli/Documents/semio'
class Node:
    def __init__(s,file,name,parent,text,sk,items):
        s.file=file;s.name=name;s.parent=parent;s.text=text;s.sk=sk;s.items=items;s.children={};s.defs={};s.uses=[];s.base=None;s.aliases={}
class Model:
    world=None
    macro_last=False
    def __init__(s,lib,ident=None,depmap=None):
        s.lib=lib;s.ident=ident;s.depmap=depmap or {};s.nodes=[];s.root=s.load(lib,None,'crate',None)
    def load(s,file,parent,name,inline=None,base=None):
        if inline is None:
            try: t,sk,items=parse(file)
            except Exception: return None
        else:
            t,sk,items=inline
        n=Node(file,name,parent,t,sk,items);n.model=s;s.nodes.append(n)
        d=base or os.path.dirname(file)
        s.fill(n,items,d,file,t,sk)
        return n
    def fill(s,n,items,d,file,t,sk):
        for it in list(items):
            it['_file']=file;it['_text']=t
            if it['cfgtest'] : continue
            kw=it['kw']
            if kw=='include':
                f=os.path.normpath(os.path.join(os.path.dirname(file),it['path']))
                if os.path.isfile(f):
                    try: t2,sk2,items2=parse(f)
                    except Exception: continue
                    s.fill(n,items2,os.path.dirname(f),f,t2,sk2)
                continue
            if kw=='mod' and it.get('name'):
                if it.get('inline'):
                    a=sk.find('{',it['start'],it['end']);b=sk.rfind('}',it['start'],it['end'])
                    body=t[a+1:b];bsk=sk[a+1:b]
                    t2,sk2,items2=parse_text(body,bsk)
                    sub=Node(file,it['name'],n,t2,sk2,items2);sub.model=s;sub.vis=it['vis'];s.nodes.append(sub)
                    sd=os.path.normpath(os.path.join(d,it['path'] if it['path'] else it['name']))
                    s.fill(sub,items2,sd,file,t2,sk2)
                    n.children[it['name']]=sub
                    continue
                f=os.path.normpath(os.path.join(d,it['path'])) if it['path'] else None
                if not f:
                    base=os.path.splitext(os.path.basename(file))[0]
                    for cand in (os.path.join(d,it['name']+'.rs'),os.path.join(d,it['name'],'mod.rs')):
                        if os.path.isfile(cand): f=cand;break
                if f and os.path.isfile(f):
                    c=s.load(f,n,it['name'])
                    if c: n.children[it['name']]=c;c.vis=it['vis']
            elif kw in('struct','enum','union','trait','type','fn','static','const') and it.get('name'):
                n.defs.setdefault(it['name'],[]).append(it)
            elif kw=='macro_rules!' and it.get('name'):
                n.defs.setdefault(it['name'],[]).append(it)
                if 'macro_export' in it.get('attrs',''):
                    it['export']=True
                    r=n
                    while r.parent is not None: r=r.parent
                    if r is not n: r.defs.setdefault(it['name'],[]).append(it)
            elif kw=='use':
                for p,al in use_leaves(it['use']): n.uses.append((it['vis'],p,al,it))
            elif kw=='extern crate':
                m2=re.match(r'^(pub\s+)?\s*(\w+)\s+as\s+(\w+)\s*$',it.get('ec',''))
                m3=re.match(r'^(\w+)\s+as\s+(\w+)\s*$',it.get('ec',''))
                if m3:
                    n.aliases[m3.group(2)]=m3.group(1)
                    if n.parent is not None: n.uses.append((it['vis'] or 'pub(crate)',('crate',) if m3.group(1)=='self' else (m3.group(1),),m3.group(2),it))
    def lines(s,node,it): return it.get('_text',node.text).count('\n',it['start'],it['end'])+1
    def resolve_in(s,node,segs,pub_only,seen=None,from_outside=True):
        """resolve segs relative to node's scope; returns ('def',node,item)|('mod',node)|('ext',segs)|None"""
        seen=seen or set()
        if not segs: return ('mod',node)
        key=(id(node),tuple(segs),pub_only)
        if key in seen: return None
        seen.add(key)
        h,rest=segs[0],list(segs[1:])
        root=node
        while root.parent is not None: root=root.parent
        if h in root.aliases and h not in node.children and h not in node.defs:
            tgt=root.aliases[h]
            if tgt=='self' or (s.ident and tgt==s.ident): return s.resolve_in(s.root,rest,False,seen) if rest else ('mod',s.root)
            q=s.tr((tgt,)+tuple(rest))
            if s.ident and q[0]==s.ident: return None
            return ('ext',q)
        if h=='crate': return s.resolve_in(s.root,rest,False,seen) if rest else ('mod',s.root)
        if h=='self': return s.resolve_in(node,rest,False,seen) if rest else ('mod',node)
        if h=='super': return s.resolve_in(node.parent,rest,False,seen) if node.parent and rest else (('mod',node.parent) if node.parent else None)
        if h in node.children and (not pub_only or node.children[h].vis.startswith('pub')) and not (not rest and Model.macro_last):
            c=node.children[h]
            return s.resolve_in(c,rest,pub_only,seen) if rest else ('mod',c)
        if h in node.defs and (not pub_only or any(i['pub'] or i.get('export') for i in node.defs[h])):
            it=[i for i in node.defs[h] if i['pub'] or i.get('export') or not pub_only][0]
            return ('def',node,it)
        for vis,p,al,it in node.uses:
            if pub_only and not vis.startswith('pub'): continue
            nm=al or p[-1]
            if nm==h and p[-1]!='*' and not (not rest and Model.macro_last and len(p)==1):
                r=s.resolve_in(node,list(p),False,seen) if p[0] in('crate','self','super') or p[0] in node.children or p[0] in node.defs else s.ext_or_local(node,p,seen)
                if r is None:
                    rt=node
                    while rt.parent is not None: rt=rt.parent
                    if rt.aliases.get(p[0])=='self': continue
                    r=('ext',s.tr(p))
                if r[0]=='mod' and rest: return s.resolve_in(r[1],rest,pub_only,seen)
                if rest and r[0]=='ext': return ('ext',tuple(r[1])+tuple(rest))
                return r
        for vis,p,al,it in node.uses:
            if pub_only and not vis.startswith('pub'): continue
            if p[-1]=='*':
                tgt=s.resolve_in(node,list(p[:-1]),False,seen) if p[0] in('crate','self','super') or p[0] in node.children else s.ext_or_local(node,p[:-1],seen)
                if tgt and tgt[0]=='mod':
                    r=s.resolve_in(tgt[1],segs,True,seen)
                    if r: return r
                elif tgt and tgt[0]=='ext' and s.world is not None:
                    cand=s.tr(tuple(tgt[1])+tuple(segs))
                    if s.world.exists(cand): return ('ext',cand)
        return None
    def ext_or_local(s,node,p,seen):
        rt=node
        while rt.parent is not None: rt=rt.parent
        if rt.aliases.get(p[0]) in ('self',s.ident) and rt.aliases.get(p[0]) is not None and p[0] not in node.children and p[0] not in node.defs:
            return s.resolve_in(s.root,list(p[1:]),False,seen) if len(p)>1 else ('mod',s.root)
        if p[0] in node.children or p[0] in node.defs: return s.resolve_in(node,list(p),False,seen)
        for vis,q,al,it in node.uses:
            nm=al or q[-1]
            if nm==p[0] and q[-1]!='*' and tuple(q)!=tuple(p) and len(q)>=1:
                r=s.resolve_in(node,list(q)+list(p[1:]),False,seen) if q[0] in('crate','self','super') or q[0] in node.children or q[0] in node.defs else s.ext_or_local(node,tuple(q)+tuple(p[1:]),seen)
                if r is not None: return r
        if p[0] not in s.depmap and not p[0].startswith('semio_') and p[0] not in ('std','core','alloc'):
            r0=s.resolve_in(node,[p[0]],False,seen)
            if r0 is not None:
                if r0[0]=='mod': return s.resolve_in(r0[1],list(p[1:]),True,seen) if len(p)>1 else r0
                if r0[0]=='ext': return ('ext',tuple(r0[1])+tuple(p[1:]))
        q=s.tr(p)
        if s.ident and q and q[0]==s.ident: return None
        return ('ext',q)
    GLOBAL_ALIAS={'protocol':'replication'}
    def tr(s,p):
        p=tuple(p)
        if p and p[0] in s.GLOBAL_ALIAS and p[0] not in s.depmap and s.GLOBAL_ALIAS[p[0]] in s.depmap.values(): return (s.GLOBAL_ALIAS[p[0]],)+p[1:]
        if p and p[0] in s.depmap and s.depmap[p[0]]!=p[0]: return (s.depmap[p[0]],)+p[1:]
        return p
    def public_names(s):
        """enumerate public names of the crate root: (name, kind, node, item) for defs, ('ext') for re-exports of other crates."""
        out={}
        r=s.root
        for nm,its in r.defs.items():
            for i in its:
                if i['pub'] or i['kw']=='macro_rules!' and i.get('export'): out[nm]=('def',r,i)
        for nm,c in r.children.items():
            if c.vis.startswith('pub'): out[nm]=('mod',c)
        for vis,p,al,it in r.uses:
            if vis.startswith('pub') and p[-1]!='*': 
                nm=al or p[-1]
                if nm=='self': continue
                res=s.resolve_in(r,[nm],True)
                out[nm]=res if res else ('ext',tuple(p))
            elif vis.startswith('pub') and p[-1]=='*':
                tgt=s.resolve_in(r,list(p[:-1]),False) if p[0] in('crate','self','super') or p[0] in r.children else s.ext_or_local(r,p[:-1],set())
                if tgt and tgt[0]=='mod':
                    for nm,its in tgt[1].defs.items():
                        for i in its:
                            if i['pub']: out.setdefault(nm,('def',tgt[1],i))
                    for nm,c in tgt[1].children.items():
                        if c.vis.startswith('pub'): out.setdefault(nm,('mod',c))
                elif tgt and tgt[0]=='ext': out.setdefault('*'+'::'.join(p[:-1]),('ext',tuple(p)))
        return out

class World:
    """Cross-crate resolver: ident -> Model (lazy). depmaps: ident -> {dependency key ident: lib ident of the target}."""
    def __init__(s,ident2lib,depmaps=None):
        s.i2l=ident2lib;s.models={};s.depmaps=depmaps or {}
    def model_for(s,ident):
        if ident in s.models: return s.models[ident]
        lib=s.i2l.get(ident);m=None
        if lib and os.path.isfile(lib):
            Model.world=s
            s.models[ident]=None
            m=Model(lib,ident,s.depmaps.get(ident))
        s.models[ident]=m;return m
    def exists(s,cand):
        m=s.model_for(cand[0])
        if m is None: return False
        rest=list(cand[1:])
        if not rest: return True
        return s.resolve(m,rest,hop=False) is not None
    def resolve(s,model,segs,hop=True):
        """follow ext chains to the defining crate. returns dict or None."""
        first=None;cur=model;segs=list(segs);n=0;chain=[];steps=[]
        while n<30:
            n+=1
            r=cur.resolve_in(cur.root,segs,True)
            if r is None: return None
            if r[0]=='ext':
                p=list(r[1]);ident=p[0]
                m=s.model_for(ident)
                if m is None: return {'kind':'external','owner':ident,'hop':first or ident,'chain':chain,'steps':steps,'path':'::'.join(p)}
                if first is None: first=ident
                chain.append(ident);steps.append([ident]+list(p[1:]))
                cur=m;segs=p[1:]
                if not segs: return {'kind':'crate','owner':ident,'hop':first,'chain':chain,'steps':steps,'file':os.path.relpath(m.lib,ROOT),'name':ident,'lines':0}
                continue
            node=r[1]
            if r[0]=='mod': return {'kind':'mod','owner':node.model.ident,'hop':first or node.model.ident,'chain':chain,'steps':steps,'file':os.path.relpath(node.file,ROOT),'name':node.name,'lines':0}
            it=r[2]
            return {'kind':'def','owner':node.model.ident,'hop':first or node.model.ident,'chain':chain,'steps':steps,'file':os.path.relpath(it['_file'],ROOT),'name':it['name'],'lines':it['_text'].count('\n',it['start'],it['end'])+1,'item_kw':it['kw']}
        return None
