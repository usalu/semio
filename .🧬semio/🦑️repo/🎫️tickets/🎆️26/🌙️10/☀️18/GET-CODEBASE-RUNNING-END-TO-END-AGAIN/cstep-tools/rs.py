"""cstep: tiny Rust lexer helpers. mask() blanks comments/strings/chars (same length) so brace and token scans are safe."""
import re
def mask(s):
    out=list(s);i=0;n=len(s)
    def blank(a,b):
        for k in range(a,b):
            if out[k]!='\n': out[k]=' '
    while i<n:
        c=s[i]
        if c=='/' and i+1<n and s[i+1]=='/':
            j=s.find('\n',i)
            j=n if j<0 else j
            blank(i,j);i=j
        elif c=='/' and i+1<n and s[i+1]=='*':
            d=1;j=i+2
            while j<n and d>0:
                if s.startswith('/*',j): d+=1;j+=2
                elif s.startswith('*/',j): d-=1;j+=2
                else: j+=1
            blank(i,j);i=j
        elif c=='r' and re.match(r'r#*"',s[i:i+12]) and (i==0 or not (s[i-1].isalnum() or s[i-1]=='_')):
            m=re.match(r'r(#*)"',s[i:]);h=m.group(1);end='"'+h
            j=s.find(end,i+len(m.group(0)))
            j=n if j<0 else j+len(end)
            blank(i+len(m.group(0))-1+0,j-1) if False else blank(i+1,j-1) if j-1>i+1 else None;i=j
        elif c=='b' and i+1<n and s[i+1]=='"':
            i+=1
        elif c=='"':
            j=i+1
            while j<n and s[j]!='"':
                j+=2 if s[j]=='\\' else 1
            blank(i+1,j);i=j+1
        elif c=="'":
            m=re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)|[^\\'\n])'",s[i:i+14])
            if m: blank(i+1,i+len(m.group(0))-1);i+=len(m.group(0))
            else: i+=1
        else: i+=1
    return ''.join(out)
def match_brace(m,i):
    d=0;n=len(m);j=i
    while j<n:
        c=m[j]
        if c in '{([': d+=1
        elif c in '})]':
            d-=1
            if d==0: return j
        j+=1
    return -1

def fn_items(s,m):
    """yield (name,start,sig_end,body_open,body_close,sig_text) for every fn with a body, using masked text m."""
    for f in re.finditer(r'\bfn\s+(\w+)',m):
        i=f.end();d=0;j=i
        n=len(m);bo=-1
        while j<n:
            c=m[j]
            if c in '([<': 
                if c=='<': 
                    d+=0
                else: d+=1
            elif c in ')]': d-=1
            elif c=='{' and d==0: bo=j;break
            elif c==';' and d==0: break
            j+=1
        if bo<0: continue
        be=match_brace(m,bo)
        yield (f.group(1),f.start(),bo,be,s[f.start():bo])

def enclosing_impl(m,pos):
    """return (header_text,start,open,close) of the innermost `impl ... {` block containing pos."""
    best=None
    for im in re.finditer(r'\bimpl\b[^{;]*?\{',m):
        b=im.end()-1
        if b>pos: break
        e=match_brace(m,b)
        if e>=pos: best=(m[im.start():b],im.start(),b,e)
    return best
