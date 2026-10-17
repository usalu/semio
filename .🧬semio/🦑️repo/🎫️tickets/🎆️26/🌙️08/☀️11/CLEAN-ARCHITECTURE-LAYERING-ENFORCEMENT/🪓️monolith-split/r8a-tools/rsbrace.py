import re
def skip(t,i):
    """returns index after a non-code span starting at i, or i if none"""
    n=len(t)
    if t.startswith('//',i):
        j=t.find('\n',i); return n if j<0 else j
    if t.startswith('/*',i):
        d=1;j=i+2
        while j<n and d:
            if t.startswith('/*',j): d+=1;j+=2
            elif t.startswith('*/',j): d-=1;j+=2
            else: j+=1
        return j
    m=re.compile(r'b?r(#*)"').match(t,i)
    if m:
        end='"'+m.group(1); j=t.find(end,m.end()); return n if j<0 else j+len(end)
    if t[i]=='"' or (t[i]=='b' and t.startswith('b"',i)):
        j=i+(2 if t[i]=='b' else 1)
        while j<n and t[j]!='"':
            j+=2 if t[j]=='\\' else 1
        return j+1
    if t[i]=="'" :
        m=re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'])'").match(t,i)
        if m: return m.end()
        return i+1  # lifetime tick
    return i
def match_brace(t,i):
    """i at '{' -> index of matching '}'"""
    d=0;n=len(t);j=i
    while j<n:
        k=skip(t,j)
        if k!=j: j=k;continue
        c=t[j]
        if c=='{':d+=1
        elif c=='}':
            d-=1
            if d==0:return j
        j+=1
    raise Exception('unbalanced')
def item_end(t,start):
    """end index (exclusive, after newline) of the item starting at start: first top-level '{...}' or ';'"""
    n=len(t);j=start;paren=0
    while j<n:
        k=skip(t,j)
        if k!=j: j=k;continue
        c=t[j]
        if c in '([': paren+=1
        elif c in ')]': paren-=1
        elif c=='{' and paren==0:
            e=match_brace(t,j)+1
            if e<n and t[e]=='\n': e+=1
            return e
        elif c==';' and paren==0:
            e=j+1
            if e<n and t[e]=='\n': e+=1
            return e
        j+=1
    raise Exception('no end')
