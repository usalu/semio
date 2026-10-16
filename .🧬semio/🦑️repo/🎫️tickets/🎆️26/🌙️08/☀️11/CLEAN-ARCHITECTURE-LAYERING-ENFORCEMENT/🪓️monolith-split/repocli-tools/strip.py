import re
def strip(t):
    out=[];i=0;n=len(t)
    while i<n:
        c=t[i]
        if t.startswith('//',i):
            j=t.find('\n',i); j=n if j<0 else j
            out.append(' '*(j-i)); i=j
        elif t.startswith('/*',i):
            j=t.find('*/',i); j=n if j<0 else j+2
            out.append(re.sub(r'[^\n]',' ',t[i:j])); i=j
        elif c=='r' and re.match(r'r#*"',t[i:i+20]) and (i==0 or not (t[i-1].isalnum() or t[i-1]=='_')):
            m=re.match(r'r(#*)"',t[i:]); h=m.group(1); end='"'+h
            j=t.find(end,i+len(m.group(0))); j=n if j<0 else j+len(end)
            out.append(re.sub(r'[^\n]',' ',t[i:j])); i=j
        elif c=='"':
            j=i+1
            while j<n and t[j]!='"':
                j+=2 if t[j]=='\\' else 1
            j+=1
            out.append('"'+re.sub(r'[^\n]',' ',t[i+1:j-1])+'"'); i=j
        elif c=="'" :
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[i:])
            if m:
                out.append(' '*len(m.group(0))); i+=len(m.group(0))
            else:
                out.append(c); i+=1
        else:
            out.append(c); i+=1
    return ''.join(out)
