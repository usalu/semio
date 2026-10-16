import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rsitems import lex,items,name_kind

REGION=re.compile(r'^\s*//#(region|endregion)')
ENDREGION=re.compile(r'^\s*//#endregion')
FNLINE=re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?)((?:async\s+)?(?:unsafe\s+)?fn\s)')

class Piece:
    def __init__(self,lines,start,end,kind,name,vis,owner=None,impl=None,member=None):
        self.lines=lines;self.start=start;self.end=end;self.kind=kind;self.name=name;self.vis=vis
        self.owner=owner;self.impl=impl;self.member=member

def _ranges(text,lines,base_line=0):
    out=[]
    for it in items(text):
        v,k,nm=name_kind(it['toks'])
        a=it['first_tok_line'];b=it['end_line']
        out.append(dict(a=a,b=b,vis=v,kind=k,name=nm,attrs=it['attrs'],toks=it['toks']))
    return out

def _chunks(lines,text,lo,hi):
    """lines: full file lines (0-based list); items found in text; returns list of (a,b,meta) 0-based inclusive, plus loose lines"""
    rs=_ranges(text,lines)
    cover=[None]*len(lines)
    for i,r in enumerate(rs):
        for x in range(r['a'],r['b']+1):cover[x]=i
    res=[]
    # split leading blanks/endregions of each item
    prev_end=-1
    for i,r in enumerate(rs):
        a=r['a']
        pre=[]
        while a<=r['b'] and (lines[a].strip()=='' or ENDREGION.match(lines[a])):
            if lines[a].strip()!='':pre.append(a)
            a+=1
        r['pre_end']=pre
        r['a2']=a
    return rs

def parse_file(path,lines=None):
    text=open(path,encoding='utf-8').read() if lines is None else '\n'.join(lines)
    ls=text.split('\n')
    return text,ls,_ranges(text,ls)

def build_pieces(ls,rs,seg_lo=0,seg_hi=None):
    """turn item ranges into ordered pieces; endregion lines at item start become separate 'endmark' pieces;
    uncovered non-blank lines become 'loose' pieces."""
    if seg_hi is None:seg_hi=len(ls)-1
    cover=[False]*len(ls)
    pcs=[]
    for r in rs:
        a=r['a'];b=r['b']
        while a<=b and (ls[a].strip()=='' or ENDREGION.match(ls[a])):
            if ENDREGION.match(ls[a]):
                pcs.append(Piece([ls[a]],a,a,'endmark','',''))
                cover[a]=True
            a+=1
        for x in range(r['a'],r['b']+1):cover[x]=True
        pcs.append(Piece(ls[a:b+1],a,b,r['kind'],r['name'],r['vis']))
        pcs[-1].meta=r
    for x in range(seg_lo,seg_hi+1):
        if not cover[x] and ls[x].strip()!='':
            pcs.append(Piece([ls[x]],x,x,'endmark' if ENDREGION.match(ls[x]) else 'loose','',''))
    pcs.sort(key=lambda p:p.start)
    return pcs

def impl_members(ls,piece):
    """for an impl piece: returns (header_lines, member_pieces, tail_lines) where members have lines/start/end (absolute 0-based)"""
    toks=piece.meta['toks']
    br=[q for q,t in enumerate(toks) if t[0]=='p' and t[1]=='{'][0]
    # absolute line of the brace
    text='\n'.join(ls)
    # compute from lines: find header end by line containing the opening brace token: use source offset
    off=toks[br][4]
    # toks offsets are relative to the text passed to items(); piece.meta toks came from whole-file parse, so offsets are file-absolute
    pre=text[:off]
    brace_line=pre.count('\n')
    endq=len(toks)-1
    endoff=toks[endq][4]
    end_line=text[:endoff].count('\n')
    header=ls[piece.start:brace_line+1]
    body_text='\n'.join(ls[brace_line:end_line+1])
    first_col=len(ls[brace_line])  # unused
    inner_start_off=off+1
    sub=text[inner_start_off:endoff]
    sub_line0=brace_line  # sub starts at the brace line (same line)
    members=[]
    mrs=[]
    for it in items(sub):
        v,k,nm=name_kind(it['toks'])
        tk=[(t[0],t[1],t[2],t[3],t[4]+inner_start_off) for t in it['toks']]
        mrs.append(dict(a=it['first_tok_line']+sub_line0,b=it['end_line']+sub_line0,vis=v,kind=k,name=nm,attrs=it['attrs'],toks=tk))
    return header,mrs,brace_line,end_line

def impl_piece_members(ls,piece):
    header,mrs,brace_line,end_line=impl_members(ls,piece)
    # normalise: member ranges may start on brace line when first token follows brace; assume not
    pcs=[]
    cover=set()
    for r in mrs:
        a=r['a'];b=r['b']
        while a<=b and (ls[a].strip()=='' or ENDREGION.match(ls[a])):
            if ENDREGION.match(ls[a]):
                pcs.append(Piece([ls[a]],a,a,'endmark','',''));cover.add(a)
            a+=1
        for x in range(r['a'],r['b']+1):cover.add(x)
        p=Piece(ls[a:b+1],a,b,r['kind'],r['name'],r['vis']);p.meta=r;pcs.append(p)
    for x in range(brace_line+1,end_line):
        if x not in cover and ls[x].strip()!='':
            pcs.append(Piece([ls[x]],x,x,'endmark' if ENDREGION.match(ls[x]) else 'loose','',''))
    pcs.sort(key=lambda p:p.start)
    return header,pcs,brace_line,end_line

def resolve_markers(pcs):
    """assign owner for endmark/loose pieces: endmark -> previous owner; loose -> next owner. pcs have .owner set for real items."""
    n=len(pcs)
    for i,p in enumerate(pcs):
        if p.owner is None and p.kind=='endmark':
            j=i-1
            while j>=0 and pcs[j].owner is None:j-=1
            assert j>=0,('endmark with no previous owner',p.lines)
            p.owner=pcs[j].owner
    for i in range(n-1,-1,-1):
        p=pcs[i]
        if p.owner is None and p.kind=='loose':
            j=i+1
            while j<n and pcs[j].owner is None:j+=1
            assert j<n,('loose with no next owner',p.lines)
            p.owner=pcs[j].owner

def insert_vis(lines,vis='pub(super) '):
    """insert vis before the first fn line that has no visibility; returns new lines"""
    out=list(lines)
    for i,l in enumerate(out):
        m=FNLINE.match(l)
        if m:
            if m.group(2):return out
            out[i]=m.group(1)+vis+l[len(m.group(1)):]
            return out
        if l.strip().startswith('//') or l.strip().startswith('#[') or l.strip()=='' :continue
    raise Exception('no fn line in '+'|'.join(lines[:3]))

def sig_of(piece,ls):
    """signature text (from fn keyword line up to just before the body brace) and param names"""
    toks=piece.meta['toks']
    # find 'fn' token
    fi=[q for q,t in enumerate(toks) if t[0]=='id' and t[1]=='fn'][0]
    depth=0;bq=None
    for q in range(fi,len(toks)):
        t=toks[q]
        if t[0]=='p' and t[1] in '([':depth+=1
        elif t[0]=='p' and t[1] in ')]':depth-=1
        elif t[0]=='p' and t[1]=='{' and depth==0:
            bq=q;break
        elif t[0]=='p' and t[1]==';' and depth==0:
            bq=q;break
    assert bq is not None
    text='\n'.join(ls)
    # signature start: start of line holding 'async'/'fn'
    first_line=text[:toks[fi][4]].count('\n')
    # async keyword may precede fn
    k=fi
    while k>0 and toks[k-1][1] in ('async','unsafe','pub'):k-=1
    first_line=text[:toks[k][4]].count('\n')
    brace_off=toks[bq][4]
    brace_line=text[:brace_off].count('\n')
    sig_lines=ls[first_line:brace_line+1]
    # cut the last line at the brace
    last=sig_lines[-1]
    col=brace_off-(text.rfind('\n',0,brace_off)+1)
    sig_lines=sig_lines[:-1]+[last[:col].rstrip()]
    # params
    po=[q for q in range(fi,bq) if toks[q][0]=='p' and toks[q][1]=='('][0]
    d=0;segs=[];cur=[];q=po
    while q<bq:
        t=toks[q]
        if t[0]=='p' and t[1] in '([{':
            d+=1
            if d==1:q+=1;continue
        elif t[0]=='p' and t[1] in ')]}':
            d-=1
            if d==0:
                segs.append(cur);break
        elif t[0]=='p' and t[1]==',' and d==1:
            segs.append(cur);cur=[];q+=1;continue
        cur.append(t);q+=1
    names=[];recv=None
    for s in segs:
        s=[t for t in s if t[0]!='comment']
        if not s:continue
        ids=[t[1] for t in s]
        if 'self' in ids and ':' not in ids:
            recv=''.join(ids);continue
        # pattern before first ':' at depth 0
        pat=[];dd=0
        for t in s:
            if t[0]=='p' and t[1]==':' and dd==0:break
            pat.append(t[1])
        names.append(''.join(pat))
    is_async='async' in [toks[x][1] for x in range(k,fi)]
    return sig_lines,names,recv,is_async,toks[fi+1][1]

def delegate_lines(piece,ls,indent='    '):
    sig,names,recv,is_async,name=sig_of(piece,ls)
    for n in names:
        assert re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*',n),('complex param',name,n)
    call='self.%s(%s)%s'%(name,', '.join(names),'.await' if is_async else '')
    out=list(sig)
    out[-1]=out[-1]+' {'
    out.append(indent*2+call)
    out.append(indent+'}')
    return out,name

def collapse(lines):
    out=[];blank=0
    for l in lines:
        if l.strip()=='':
            blank+=1
            if blank>1:continue
            out.append('')
        else:
            blank=0;out.append(l)
    while out and out[-1]=='':out.pop()
    return out
