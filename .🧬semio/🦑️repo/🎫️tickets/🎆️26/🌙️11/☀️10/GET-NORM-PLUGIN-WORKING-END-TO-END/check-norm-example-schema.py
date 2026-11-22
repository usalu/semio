import json,glob,re,os,sys
root=os.path.expanduser('~/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts')
def kebab(n):
    n=n[:1].upper()+n[1:];c=list(n);o=""
    for i,ch in enumerate(c):
        if ch in "_-":
            if o and not o.endswith("-"): o+="-"
            continue
        if ch.isupper():
            p=c[i-1] if i else None;nx=c[i+1] if i+1<len(c) else None
            b=(p is not None and (p.islower() or p.isdigit())) or (p is not None and p.isupper() and nx is not None and nx.islower())
            if b and o and not o.endswith("-"): o+="-"
            o+=ch.lower()
        else: o+=ch
    return o
SHARED={'text','locale'}
def nk(s): return re.sub(r'[-_]','',s).lower()
class P:
    def __init__(s,t): s.t=t; s.i=0
    def ws(s):
        while s.i<len(s.t) and s.t[s.i] in ' \t\r\n': s.i+=1
    def word(s):
        s.ws(); st=s.i
        while s.i<len(s.t):
            c=s.t[s.i]
            if c in ' \t\r\n': break
            if c=='"': s.quote(); continue
            if c in '[{' and s.i>st and s.t[s.i-1]=='=': s.bal(); continue
            if c in ']}' : break
            s.i+=1
        return s.t[st:s.i]
    def quote(s):
        s.i+=1
        while s.i<len(s.t) and s.t[s.i]!='"':
            s.i+= 2 if s.t[s.i]=='\\' else 1
        s.i+=1
    def bal(s):
        d=0
        while s.i<len(s.t):
            c=s.t[s.i]
            if c=='"': s.quote(); continue
            if c in '[{': d+=1
            elif c in ']}':
                d-=1
                if d==0: s.i+=1; return
            s.i+=1
    def peek(s): s.ws(); return s.t[s.i] if s.i<len(s.t) else ''
def run():
    issues=[];stats={}
    for sp in sorted(glob.glob(root+'/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🔣️.json')):
        fam=sp[len(root)+1:].split('/')[0]
        sc=json.load(open(sp)); defs=sc.get('$defs',{})
        def res(n,d=0):
            if not isinstance(n,dict) or d>8: return {}
            if '$ref' in n: return res(defs.get(n['$ref'].split('/')[-1],{}),d+1)
            for k in ('anyOf','oneOf','allOf'):
                if k in n:
                    c=[res(x,d+1) for x in n[k] if x.get('type')!='null']
                    if len(c)==1: return c[0]
            return n
        gp={}
        rtxt=''.join(open(x).read() for x in glob.glob(os.path.join(os.path.dirname(sp),'📸️snapshot','**','🦀️.rs'),recursive=True))
        rfields=set(nk(x) for x in re.findall(r'pub ([a-z_0-9]+):',rtxt))
        dslkeys=set(re.findall(r'dsl\(key = "([^"]+)"',rtxt))
        def collect(n):
            if isinstance(n,dict):
                for k,v in (n.get('properties') or {}).items():
                    o=gp.get(nk(k))
                    if o is None or (res(v).get('type')=='array' and res(o).get('type')!='array'): gp[nk(k)]=v
                for v in n.values(): collect(v)
            elif isinstance(n,list):
                for v in n: collect(v)
        collect(sc)
        def enumv(n):
            n=res(n)
            return n.get('enum') if isinstance(n,dict) else None
        def items(n):
            n=res(n)
            return res(n.get('items')) if n.get('type')=='array' else n
        base=os.path.dirname(os.path.dirname(sp))
        for f in sorted(glob.glob(base+'/🖼️assets/**/🗣️.dsl.semio',recursive=True)):
            name=os.path.basename(os.path.dirname(f)); tag=f'{fam} {name}'
            stats[fam]=stats.get(fam,0)+1
            def err(m): issues.append(f'{tag}: {m}')
            txt=open(f).read(); lines=txt.split('\n')
            def checkenum(prop,val,v,ctx):
                ev=enumv(prop)
                if ev is None or val is None: return
                val=val.strip('"')
                if val=='_' or val in dslkeys: return
                if val in [kebab(str(e)) for e in ev]: return
                err(f'{ctx}: enum value {val!r} not in {[kebab(str(e)) for e in ev]}')
            def props(d): return {nk(k):(k,v) for k,v in (d.get('properties') or {}).items()}
            def checkkeys(d,keys,ctx,vals=None):
                pr=props(d); have=set()
                for k in keys:
                    if nk(k) not in pr and nk(k) not in rfields: err(f'{ctx}: unknown field {k!r}')
                    else: have.add(nk(k))
                for r in d.get('required',[]):
                    if nk(r) not in have: err(f'{ctx}: missing required field {kebab(r)!r}')
                if vals:
                    for k,v in vals.items():
                        if nk(k) in pr: checkenum(pr[nk(k)][1],v,v,f'{ctx}.{k}')
            def parse_pairs(p,d,ctx,end):
                keys=[];vals={}
                while True:
                    c=p.peek()
                    if c==''or c==end: break
                    st=p.i; w=p.word()
                    if not w: p.i+=1; continue
                    pk=p.peek()
                    if pk=='[' and '=' not in w:
                        cols=read_header(p); sec(p,d,w,cols,ctx); keys.append(w); continue
                    if '=' in w:
                        k,v=w.split('=',1)
                        if v=='' and p.peek() in '[{':
                            sub=prop_of(d,k); v2=p.i
                            val=read_cell(p,sub,'?' ,f'{ctx}.{k}')
                        else:
                            keys.append(k); vals[k]=v
                            pr=props(d).get(nk(k))
                            if pr and v.startswith('{'): pass
                    else: err(f'{ctx}: stray token {w!r}')
                    if '=' in w and (v=='' ): keys.append(k)
                return keys,vals
            def prop_of(d,k):
                pr=props(d).get(nk(k)); return pr[1] if pr else None
            def read_header(p):
                p.ws(); assert p.t[p.i]=='['; j=p.t.index(']',p.i); h=p.t[p.i+1:j]; p.i=j+1
                p.ws(); assert p.t[p.i]=='{',f'section brace at {p.t[p.i-30:p.i+10]!r}'; p.i+=1
                return [tuple(c.rsplit(':',1)) for c in h.split()]
            def read_obj(p,d,ctx):
                p.ws(); assert p.t[p.i]=='{'; p.i+=1
                keys,vals=parse_pairs(p,d,ctx,'}'); p.ws(); p.i+=1
                checkkeys(d,keys,ctx,vals)
            def read_cell(p,prop,typ,ctx):
                c=p.peek()
                if c=='[':
                    p.i+=1; it=items(prop) if prop else {}
                    while p.peek() not in (']',''):
                        if p.peek()=='{':
                            if it.get('type')=='object' or it.get('properties'): read_obj(p,it,ctx)
                            else: p.bal()
                        else: p.word()
                    p.i+=1; return None
                if c=='{':
                    d=res(prop) if prop else {}
                    if d.get('properties'): read_obj(p,d,ctx)
                    else: p.bal()
                    return None
                if c=='"': st=p.i; p.quote(); return p.t[st:p.i]
                return p.word()
            def sec(p,d,sname,cols,ctx):
                pr=props(d).get(nk(sname))
                if not pr and nk(sname) in gp: pr=(sname,gp[nk(sname)])
                if not pr and nk(sname) in rfields: pr=(sname,{}); 
                if not pr: err(f'{ctx}: unknown table {sname!r}'); it={}
                else: it=items(pr[1])
                ids=props(it)
                for c,t in cols:
                    if nk(c) not in ids and ids and nk(c) not in rfields: err(f'{ctx}.{sname}: unknown column {c!r}')
                for r in it.get('required',[]):
                    if nk(r) not in [nk(c) for c,_ in cols]: err(f'{ctx}.{sname}: missing required column {kebab(r)!r}')
                n=0
                while True:
                    if p.peek()=='}': p.i+=1; break
                    if p.peek()=='': err(f'{ctx}.{sname}: unterminated'); return
                    for c,t in cols:
                        cp=ids.get(nk(c)); cp=cp[1] if cp else None
                        v=read_cell(p,cp,t,f'{ctx}.{sname}[{n}].{c}')
                        if t=='ENUM' and v is not None: checkenum(cp,v,v,f'{ctx}.{sname}[{n}].{c}')
                    n+=1
            try:
                if not lines[0].startswith('semio norm.'): err('bad header line')
                p=P('\n'.join(lines[1:])); top=sc
                keys=[]; vals={}
                while p.peek()!='':
                    st=p.i; w=p.word()
                    if p.peek()=='[' and '=' not in w:
                        cols=read_header(p); sec(p,top,w,cols,'doc'); keys.append(w)
                    elif '=' in w:
                        k,v=w.split('=',1); keys.append(k); vals[k]=v
                    else: err(f'stray token {w!r}');
                flat=set(re.findall(r'(?<![a-z0-9-])([a-z0-9-]+)=','\n'.join(lines[1:3])))
                checkkeys(top,[k for k in keys if nk(k) in props(top)]+[k for k in flat if nk(k) in props(top)] ,'doc',vals)
                for k in keys:
                    if nk(k) not in props(top) and nk(k) not in gp and nk(k) not in rfields and nk(k) not in SHARED: err(f'doc: unknown top-level field {k!r}')
            except Exception as e:
                err(f'parser error {type(e).__name__}: {e}')
    return issues,stats
if __name__=='__main__':
    issues,stats=run()
    seen=[]
    for i in issues:
        if i not in seen: seen.append(i)
    for i in seen: print(i)
    print('families',len(stats),'assets',sum(stats.values()),'issues',len(seen))
    sys.exit(1 if seen else 0)
