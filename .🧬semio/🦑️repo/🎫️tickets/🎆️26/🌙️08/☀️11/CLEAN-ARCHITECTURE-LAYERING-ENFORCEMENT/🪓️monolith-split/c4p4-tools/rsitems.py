import re,sys,json

def lex(src):
    n=len(src);i=0;toks=[];line=0
    # returns list of (kind,text,start_line,end_line,start_idx)
    while i<n:
        c=src[i]
        if c=='\n':
            line+=1;i+=1;continue
        if c in ' \t\r':
            i+=1;continue
        if src.startswith('//',i):
            j=src.find('\n',i)
            if j<0:j=n
            toks.append(('comment',src[i:j],line,line,i));i=j;continue
        if src.startswith('/*',i):
            depth=1;j=i+2;sl=line
            while j<n and depth:
                if src.startswith('/*',j):depth+=1;j+=2
                elif src.startswith('*/',j):depth-=1;j+=2
                else:
                    if src[j]=='\n':line+=1
                    j+=1
            toks.append(('comment',src[i:j],sl,line,i));i=j;continue
        m=re.match(r'b?r(#*)"',src[i:i+40])
        if m:
            h=m.group(1);end='"'+h
            j=src.find(end,i+m.end())
            j+=len(end)
            sl=line;line+=src[i:j].count('\n')
            toks.append(('str',src[i:j],sl,line,i));i=j;continue
        if c=='"' or (c=='b' and src.startswith('b"',i)):
            j=i+(2 if c=='b' else 1);sl=line
            while True:
                if src[j]=='\\':
                    if src[j+1]=='\n':line+=1
                    j+=2;continue
                if src[j]=='"':j+=1;break
                if src[j]=='\n':line+=1
                j+=1
            toks.append(('str',src[i:j],sl,line,i));i=j;continue
        if c=="'" or src.startswith("b'",i):
            k=i+(1 if c=="'" else 2)
            m=re.match(r"\\(x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)'",src[k:k+14])
            if m:
                toks.append(('char',src[i:k+m.end()],line,line,i));i=k+m.end();continue
            if src[k+1:k+2]=="'" and src[k]!='\\':
                toks.append(('char',src[i:k+2],line,line,i));i=k+2;continue
            m=re.match(r"[A-Za-z_][A-Za-z0-9_]*",src[k:])
            toks.append(('life',src[i:k+(m.end() if m else 0)],line,line,i));i=k+(m.end() if m else 0);continue
        m=re.match(r'[A-Za-z_][A-Za-z0-9_]*',src[i:i+200])
        if m:
            toks.append(('id',m.group(0),line,line,i));i+=m.end();continue
        if c.isdigit():
            m=re.match(r'[0-9][0-9A-Za-z_\.]*',src[i:])
            t=m.group(0)
            if t.endswith('.'):t=t[:-1]
            if '..' in t:t=t.split('..')[0]
            toks.append(('num',t,line,line,i));i+=len(t);continue
        if ord(c)>127:
            # emoji identifiers in strings only; treat as punct
            toks.append(('p',c,line,line,i));i+=1;continue
        toks.append(('p',c,line,line,i));i+=1
    return toks

def items(src):
    toks=lex(src)
    lines=src.split('\n')
    depth=0
    chunks=[]
    cur_start_tok=0
    out=[]
    start_line=None
    first=True
    # walk tokens; item ends at '}' bringing depth 0 or ';' at depth 0
    i=0;t0=0
    seg=[]
    for idx,t in enumerate(toks):
        k,x,sl,el,si=t
        if k=='p':
            if x in '{([':depth+=1
            elif x in '})]':
                depth-=1
                if depth==0 and x=='}':
                    nxt=toks[idx+1][1] if idx+1<len(toks) else ''
                    if nxt not in (';','>'):
                        out.append((t0,idx));t0=idx+1
            elif x==';' and depth==0:
                out.append((t0,idx));t0=idx+1
    res=[]
    for a,b in out:
        sub=toks[a:b+1]
        # first non-comment non-attr token index
        j=0;attrs=[]
        hdr_start=None
        while j<len(sub):
            k,x,sl,el,si=sub[j]
            if k=='comment':j+=1;continue
            if x=='#' and k=='p':
                # attribute: skip to matching ]
                d=0;s=j;j+=1
                if sub[j][1]=='!':j+=1
                while True:
                    if sub[j][1]=='[':d+=1
                    elif sub[j][1]==']':
                        d-=1
                        if d==0:break
                    j+=1
                attrs.append(''.join(l[1] for l in sub[s:j+1]))
                j+=1;continue
            break
        hs=sub[j][2] if j<len(sub) else sub[0][2]
        res.append(dict(first_tok_line=sub[0][2],hdr_line=hs,end_line=sub[-1][3],attrs=attrs,toks=sub[j:],attr_ids={t[1] for t in sub[:j] if t[0]=='id'}))
    return res

def name_kind(toks):
    # toks starting at header
    ids=[t for t in toks if True]
    i=0
    vis=''
    if toks[i][1]=='pub':
        vis='pub'
        i+=1
        if toks[i][1]=='(':
            j=i
            while toks[j][1]!=')':j+=1
            vis='pub'+''.join(t[1] for t in toks[i:j+1])
            i=j+1
    while toks[i][1] in ('async','unsafe','extern','default') or toks[i][0]=='str' or (toks[i][1]=='const' and toks[i+1][1] in ('fn','unsafe','async','extern')):
        i+=1
    kw=toks[i][1]
    if kw in('fn','struct','enum','trait','type','const','static','mod','union'):
        nm=toks[i+1][1]
        if nm=='mut':nm=toks[i+2][1]
        return vis,kw,nm
    if kw=='use':return vis,'use',''
    if kw=='impl':
        # find name tokens up to '{'
        j=i+1;s=[]
        while toks[j][1]!='{' and toks[j][1]!='where':
            s.append(toks[j][1]);j+=1
        return vis,'impl',''.join(s)
    if kw=='macro_rules':return vis,'macro',toks[i+2][1]
    if kw=='include':return vis,'include',''
    return vis,kw,''

if __name__=='__main__':
    p=sys.argv[1]
    src=open(p,encoding='utf-8').read()
    its=items(src)
    for it in its:
        v,k,nm=name_kind(it['toks'])
        print(f"{it['first_tok_line']+1}\t{it['hdr_line']+1}\t{it['end_line']+1}\t{v}\t{k}\t{nm}\t{' '.join(it['attrs'])[:60]}")
