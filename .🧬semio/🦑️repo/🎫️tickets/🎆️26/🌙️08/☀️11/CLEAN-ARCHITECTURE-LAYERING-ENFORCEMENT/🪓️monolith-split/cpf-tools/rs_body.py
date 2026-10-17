import os,re,sys,json,hashlib,collections
sys.path.insert(0,os.path.dirname(__file__))
import rslex
from edit_lib import ROOT
G=os.environ["G"]
APPLY="--apply" in sys.argv
NORM=re.compile(r"\s+")
def load_h():
    H=set()
    for f in ("cpf-head-comments-a.bin","cpf-head-comments-b.bin"):
        b=open(G+"/"+f,"rb").read()
        H.update(b[i:i+8] for i in range(0,len(b),8))
    return H
def hsh(body):
    body=NORM.sub(" ",body).strip()
    body=re.sub(r"\s*\*/$","",body).strip()
    if not body: return None
    return hashlib.blake2b(body.encode("utf-8"),digest_size=8).digest()
EXEMPT=re.compile(r"^(//|/\*)\s*(#?(end)?region\b|SAFETY\b|safety\b|~|rustfmt|clippy::|cargo|@ts-|eslint|prettier|nolint)",re.I)
def comment_lines(c):
    t=c['text']
    if not c['block']:
        m=re.match(r"//[/!]?(.*)$",t); return [m.group(1)]
    out=[]
    for ln in t.split("\n"):
        ln=re.sub(r"^\s*(?:/\*+|\*+(?!/))","",ln.strip())
        ln=re.sub(r"\*/\s*$","",ln)
        out.append(ln)
    return out
def candidates(path,text,H):
    cs,ok=rslex.lex(text)
    lines_start=[0]
    for m in re.finditer("\n",text): lines_start.append(m.end())
    import bisect
    def lineof(pos): return bisect.bisect_right(lines_start,pos)-1
    res=[]
    for c in cs:
        if c['doc'] or not (c['fn'] or c['de']): continue
        if EXEMPT.match(c['text'].strip()): continue
        ln=lineof(c['start']); ls=lines_start[ln]
        c['line']=ln; c['own']=(text[ls:c['start']].strip()=="")
        c['endline']=lineof(c['end']-1 if c['end']>c['start'] else c['end'])
        res.append(c)
    # runs
    runs=[]; cur=None
    for c in res:
        if cur and not c['block'] and not cur[-1]['block'] and c['own'] and cur[-1]['own'] and c['line']==cur[-1]['line']+1 and c['fn']==cur[-1]['fn']:
            cur.append(c)
        else:
            cur=[c]; runs.append(cur)
    out=[]
    for r in runs:
        hs=[hsh(l) for c in r for l in comment_lines(c)]
        hs=[h for h in hs if h]
        if not hs: continue
        if any(h in H for h in hs): continue
        out.append(r)
    return out,ok
def apply_runs(text,runs):
    # remove from end to start
    items=[]
    for r in runs:
        for c in r: items.append(c)
    items.sort(key=lambda c:c['start'],reverse=True)
    for c in items:
        s,e=c['start'],c['end']
        ls=text.rfind("\n",0,s)+1
        le=text.find("\n",e); le=len(text) if le<0 else le
        before=text[ls:s]; after=text[e:le]
        if before.strip()=="" and after.strip()=="":
            text=text[:ls]+text[le+1:] if le<len(text) else text[:ls]
        elif after.strip()=="":
            text=text[:ls]+before.rstrip()+text[le:]
        else:
            text=text[:s].rstrip(" \t")+(" " if text[s-1:s] not in (" ","\t") and after[:1] not in (" ","") else "")+text[e:].lstrip(" \t") if False else text[:s]+text[e:]
    return text
if __name__=="__main__":
    H=load_h()
    paths=[]
    for l in open(G+"/cpf-diff-name-status.txt",encoding="utf-8"):
        st,p=l.rstrip("\n").split("\t",1)
        paths.append(p)
    paths+= [l.rstrip("\n") for l in open(G+"/cpf-untracked.txt",encoding="utf-8")]
    paths=sorted(set(p for p in paths if p.endswith(".rs") and not p.startswith(".🧬semio/") and not p.startswith("Users/") and "🤖️generated" not in p))
    extra=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
    if extra: paths=[p.rstrip("\n") for p in open(extra,encoding="utf-8")]
    stats=collections.Counter(); report=[]; unbalanced=[]
    for p in paths:
        try: text=open(os.path.join(ROOT,p),encoding="utf-8").read()
        except Exception: stats['unreadable']+=1; continue
        if '//' not in text and '/*' not in text: continue
        stats['files']+=1
        runs,ok=candidates(p,text,H)
        if not ok: unbalanced.append(p)
        if not runs: continue
        nl=sum(len(r) for r in runs)
        stats['files_with']+=1; stats['comments']+=nl
        report.append((p,[(r[0]['line']+1,len(r),r[0]['text'].strip()[:90],'fn' if r[0]['fn'] else 'def') for r in runs]))
        if APPLY:
            new=apply_runs(text,runs)
            before=rslex.strip_comments(text); after=rslex.strip_comments(new)
            assert before==after,("code changed",p)
            cur=open(os.path.join(ROOT,p),encoding="utf-8").read()
            assert cur==text,("file changed concurrently",p)
            tmp=os.path.join(ROOT,p)+".cpf.tmp"; open(tmp,"w",encoding="utf-8").write(new); os.replace(tmp,os.path.join(ROOT,p))
    print(dict(stats),"unbalanced",len(unbalanced))
    json.dump(dict(report=report,unbalanced=unbalanced),open(G+("/cpf-rs-body-applied.json" if APPLY else "/cpf-rs-body-plan.json"),"w"),ensure_ascii=False,indent=1)
