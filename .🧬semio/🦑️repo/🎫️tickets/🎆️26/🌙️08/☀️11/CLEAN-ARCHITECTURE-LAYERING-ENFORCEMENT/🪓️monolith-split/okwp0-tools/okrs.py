"""Rough Rust top-level item splitter used to analyse the pack carve (not a compiler)."""
import re, sys

def tokenize(src):
    """Yield (kind, text, start, end). kinds: id, punct, str, char, life, num, comment, ws"""
    i=0;n=len(src);out=[]
    while i<n:
        c=src[i]
        if c.isspace():
            j=i
            while j<n and src[j].isspace(): j+=1
            out.append(('ws',src[i:j],i,j));i=j;continue
        if src.startswith('//',i):
            j=src.find('\n',i)
            if j<0:j=n
            out.append(('comment',src[i:j],i,j));i=j;continue
        if src.startswith('/*',i):
            d=1;j=i+2
            while j<n and d:
                if src.startswith('/*',j):d+=1;j+=2
                elif src.startswith('*/',j):d-=1;j+=2
                else:j+=1
            out.append(('comment',src[i:j],i,j));i=j;continue
        m=re.match(r'(b?r)(#*)"',src[i:i+40])
        if m:
            hashes=m.group(2);end='"'+hashes
            j=src.find(end,i+len(m.group(0)))
            j=j+len(end)
            out.append(('str',src[i:j],i,j));i=j;continue
        if c=='"' or (c=='b' and src.startswith('b"',i)):
            j=i+(2 if c=='b' else 1)
            while j<n and src[j]!='"':
                if src[j]=='\\': j+=1
                j+=1
            j+=1
            out.append(('str',src[i:j],i,j));i=j;continue
        if c=="'" or (c=='b' and src.startswith("b'",i)):
            k=i+(2 if c=='b' else 1)
            # char literal or lifetime
            m=re.match(r"'(\\.[^']*|[^\\'])'",src[k-1:k+12]) if c=="'" else re.match(r"b'(\\.[^']*|[^\\'])'",src[i:i+14])
            if m:
                j=(k-1)+len(m.group(0)) if c=="'" else i+len(m.group(0))
                out.append(('char',src[i:j],i,j));i=j;continue
            m=re.match(r"'([A-Za-z_][A-Za-z_0-9]*)",src[i:])
            if m:
                out.append(('life',m.group(0),i,i+len(m.group(0))));i+=len(m.group(0));continue
        if c.isalpha() or c=='_' or ord(c)>127:
            j=i
            while j<n and (src[j].isalnum() or src[j]=='_' or ord(src[j])>127) : j+=1
            out.append(('id',src[i:j],i,j));i=j;continue
        if c.isdigit():
            j=i
            while j<n and (src[j].isalnum() or src[j]=='_' or (src[j]=='.' and j+1<n and src[j+1].isdigit())): j+=1
            out.append(('num',src[i:j],i,j));i=j;continue
        out.append(('punct',c,i,i+1));i+=1
    return out

def items(src):
    """Split top-level items. Returns list of dict(name,kind,vis,start,end,text,attrs)."""
    toks=[t for t in tokenize(src)]
    res=[];i=0;n=len(toks)
    def nxt(k):
        while k<n and toks[k][0] in('ws',):k+=1
        return k
    depth=0
    start=None; firsttok=None
    k=0
    cur_start=0
    # iterate; item start at first non-ws token (comments included to carry docs); track depth
    idx=0
    while idx<n:
        # skip ws
        while idx<n and toks[idx][0]=='ws': idx+=1
        if idx>=n:break
        s=idx
        # consume leading comments/doc/attrs and region markers as part of item header
        # parse until item end at depth 0
        d=0;j=idx;ended=False;sawcode=False;paren=0
        while j<n:
            kd,tx,a,b=toks[j]
            if kd=='comment' or kd=='ws':
                j+=1
                continue
            if kd=='punct':
                if tx in '{([': d+=1
                elif tx in '})]':
                    d-=1
                elif tx==';' and d==0 and sawcode_kind(toks,s,j):
                    j+=1;ended=True;break
                if tx=='}' and d==0 and sawcode_kind(toks,s,j) and block_item_end(toks,s,j):
                    j+=1;ended=True;break
                if tx=='#' : pass
            j+=1
        # j is end
        text=src[toks[s][2]:toks[min(j,n)-1][3]]
        res.append(parse_item(src,toks,s,j))
        idx=j
    return res

ATTR_START=('#',)
def sawcode_kind(toks,s,j):
    # true if between s..j there is code beyond attributes/comments (i.e. not only an attribute)
    d=0;k=s;ina=False
    while k<=j:
        kd,tx,a,b=toks[k]
        if kd in('ws','comment'):k+=1;continue
        if kd=='punct' and tx=='#' :
            # attribute: skip #[...] or #![...]
            k+=1
            while toks[k][0]=='ws' or (toks[k][0]=='punct' and toks[k][1]=='!'):k+=1
            if toks[k][1]=='[':
                dd=0
                while True:
                    if toks[k][0]=='punct' and toks[k][1]=='[':dd+=1
                    elif toks[k][0]=='punct' and toks[k][1]==']':
                        dd-=1
                        if dd==0:k+=1;break
                    k+=1
                continue
        return True
    return False

def block_item_end(toks,s,j):
    # decide whether '}' at depth0 ends an item: true unless the item is e.g. `const X: T = { ... };` / `static`/`let` / use with braces / type alias, where ';' follows.
    k=s
    # find first code keyword after attrs and vis
    while k<=j:
        kd,tx,a,b=toks[k]
        if kd in('ws','comment'):k+=1;continue
        if kd=='punct' and tx=='#':
            k+=1
            while toks[k][1]!='[':k+=1
            dd=0
            while True:
                if toks[k][1]=='[' and toks[k][0]=='punct':dd+=1
                elif toks[k][1]==']' and toks[k][0]=='punct':
                    dd-=1
                    if dd==0:k+=1;break
                k+=1
            continue
        break
    words=[]
    while k<=j and len(words)<8:
        kd,tx,a,b=toks[k]
        if kd=='id':words.append(tx)
        elif kd=='punct' and tx=='(' and words and words[-1]=='pub':
            pass
        elif kd=='punct' and tx in '{;=<(:':break
        k+=1
    while words and words[0] in('pub','crate','super','in','self','unsafe','async','extern'):
        words=words[1:]
    if words and words[0]=='const' and len(words)>1 and words[1] in('fn','unsafe','async','extern'): return True
    if words and words[0] in('use','const','static','type') : return False
    if 'macro_rules' in words[:1]: return True
    # extern crate / mod foo; handled by ';'
    return True

KW={'fn','struct','enum','const','static','type','trait','mod','use','impl','macro_rules','union','extern'}
def parse_item(src,toks,s,e):
    k=s;attrs=[];docs=[];vis=''
    while k<e:
        kd,tx,a,b=toks[k]
        if kd=='comment':
            docs.append(tx);k+=1;continue
        if kd=='ws':k+=1;continue
        if kd=='punct' and tx=='#':
            st=toks[k][2];k+=1
            while toks[k][1]!='[':k+=1
            dd=0
            while True:
                if toks[k][0]=='punct' and toks[k][1]=='[':dd+=1
                elif toks[k][0]=='punct' and toks[k][1]==']':
                    dd-=1
                    if dd==0:k+=1;break
                k+=1
            attrs.append(src[st:toks[k-1][3]]);continue
        break
    # visibility
    if k<e and toks[k][1]=='pub':
        st=toks[k][2];k+=1
        k2=k
        while toks[k2][0]=='ws':k2+=1
        if toks[k2][1]=='(':
            dd=0
            while True:
                if toks[k2][1]=='(':dd+=1
                elif toks[k2][1]==')':
                    dd-=1
                    if dd==0:k2+=1;break
                k2+=1
            k=k2
        vis=src[st:toks[k-1][3]]
    kind=None;name=None
    while k<e:
        kd,tx,a,b=toks[k]
        if kd=='id' and tx in('async','unsafe','extern','default'):k+=1;continue
        if kd=='str':k+=1;continue
        if kd=='ws':k+=1;continue
        if kd=='id' and tx=='const':
            k2=k+1
            while toks[k2][0]=='ws':k2+=1
            if toks[k2][1] in('fn','unsafe','async','extern'):
                k+=1;continue
        if kd=='id' and tx in KW:
            kind=tx;k+=1
            if kind=='macro_rules':
                while toks[k][1]!='!' :k+=1
                k+=1
            while toks[k][0]=='ws':k+=1
            if kind in('fn','struct','enum','const','static','type','trait','mod','macro_rules','union'):
                name=toks[k][1]
            elif kind=='impl':
                # name = text up to '{' or 'where'
                j=k;parts=[]
                while j<e and toks[j][1] not in('{',) and toks[j][1]!='where':
                    if toks[j][0]!='ws' and toks[j][0]!='comment':parts.append(toks[j][1])
                    j+=1
                name=' '.join(parts)
            elif kind=='use':
                j=k;parts=[]
                while j<e and toks[j][1]!=';':
                    if toks[j][0] not in('ws','comment'):parts.append(toks[j][1])
                    j+=1
                name=''.join(parts)
            break
        k+=1
    return dict(kind=kind,name=name,vis=vis,start=toks[s][2],end=toks[e-1][3],attrs=attrs,text=src[toks[s][2]:toks[e-1][3]])

if __name__=='__main__':
    src=open(sys.argv[1],encoding='utf8').read()
    for it in items(src):
        print(it['kind'],it['name'] if it['kind']!='use' else '', it['vis'], src.count('\n',0,it['start'])+1, src.count('\n',0,it['end'])+1, len(it['text'].encode()))
