import os,sys,re,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind
from engine import ident_set

TOPKINDS=('fn','const','static','struct','enum','trait','type','union')
MEMBERKINDS=('fn','const','type')

def split_fields(toks,start,open_tok,close_tok):
    """toks: token list; start: index of open token; returns list of (field_name_or_None, vis_present, insert_si)"""
    depth=0;angle=0;i=start;fields=[];seg=[]
    n=len(toks)
    segs=[];cur=[]
    while i<n:
        k,x,sl,el,si=toks[i]
        if k=='p':
            if x in '{([':
                depth+=1
                if depth==1 and i==start:
                    i+=1;continue
            elif x in '})]':
                depth-=1
                if depth==0:
                    segs.append(cur);break
            elif x=='<' and depth==1:angle+=1
            elif x=='>' and depth==1 and not (i>0 and toks[i-1][1]=='-'):angle=max(0,angle-1)
            elif x==',' and depth==1 and angle==0:
                segs.append(cur);cur=[];i+=1;continue
        cur.append(toks[i]);i+=1
    out=[]
    for s in segs:
        s=[t for t in s if t[0]!='comment']
        j=0
        while j<len(s) and s[j][1]=='#' and s[j][0]=='p':
            d=0;j+=1
            while j<len(s):
                if s[j][1]=='[':d+=1
                elif s[j][1]==']':
                    d-=1
                    if d==0:j+=1;break
                j+=1
        if j>=len(s):continue
        vis=False
        if s[j][1]=='pub':
            vis=True
        name=None
        jj=j+(1 if vis else 0)
        if vis and jj<len(s) and s[jj][1]=='(':
            while s[jj][1]!=')':jj+=1
            jj+=1
        if open_tok=='{':
            if jj<len(s) and s[jj][0]=='id':name=s[jj][1]
        out.append((name,vis,s[j][4]))
    return out

def plan(text,others,kinds_skip=()):
    """returns list of (pos,insert,label)"""
    edits=[]
    for it in items(text):
        v,k,nm=name_kind(it['toks'])
        toks=it['toks']
        if k in TOPKINDS:
            if v=='' and nm in others:
                edits.append((toks[0][4],'pub(super) ',('item',k,nm)))
            if k=='struct':
                # find open token
                idx=None
                sk=[q for q,t in enumerate(toks) if t[0]=='id' and t[1]=='struct'][0]
                for q,t in enumerate(toks):
                    if q<=sk:continue
                    if t[0]=='p' and t[1] in '{(' :idx=q;break
                    if t[0]=='p' and t[1]==';':break
                if idx is not None:
                    ot=toks[idx][1]
                    fl=split_fields(toks,idx,ot,'}' if ot=='{' else ')')
                    for fname,vis,pos in fl:
                        if vis:continue
                        if ot=='{':
                            if fname and fname in others:
                                edits.append((pos,'pub(super) ',('field',nm,fname)))
                        else:
                            if nm in others:
                                edits.append((pos,'pub(super) ',('tuplefield',nm,'')))
        elif k=='impl':
            # inherent?
            br=None
            for q,t in enumerate(toks):
                if t[0]=='p' and t[1]=='{':br=q;break
            if br is None:continue
            hdr=toks[:br]
            if any(t[0]=='id' and t[1]=='for' for t in hdr):continue
            depth=0;endq=None
            for q in range(br,len(toks)):
                t=toks[q]
                if t[0]=='p' and t[1] in '{([':depth+=1
                elif t[0]=='p' and t[1] in '})]':
                    depth-=1
                    if depth==0:endq=q;break
            base=toks[br][4]+1
            sub=text[base:toks[endq][4]]
            tname=''.join(t[1] for t in hdr if t[0]!='comment')
            for mit in items(sub):
                mv,mk,mn=name_kind(mit['toks'])
                if mk in MEMBERKINDS and mv=='' and mn in others:
                    edits.append((mit['toks'][0][4]+base,'pub(super) ',('member',tname,mn)))
    return edits

def apply(text,edits):
    for pos,ins,_ in sorted(edits,key=lambda e:-e[0]):
        text=text[:pos]+ins+text[pos:]
    return text

if __name__=='__main__':
    cfg=json.load(open(sys.argv[1]))
    root=cfg['root']
    files={}
    for f in cfg['files']:
        files[f]=open(root+'/'+f,encoding='utf-8').read()
    ext={}
    for f in cfg.get('external',[]):
        ext[f]=ident_set(open(f,encoding='utf-8').read())
    idents={f:ident_set(t) for f,t in files.items()}
    report=[]
    for f in cfg['widen']:
        others=set()
        for g,s in idents.items():
            if g!=f:others|=s
        for s in ext.values():others|=s
        ed=plan(files[f],others)
        if cfg.get('dry'):
            pass
        else:
            files[f]=apply(files[f],ed)
            open(root+'/'+f,'w',encoding='utf-8').write(files[f])
        for e in ed:report.append((f,)+e[2])
    for r in report:print('\t'.join(r))
    print('TOTAL',len(report),file=sys.stderr)
