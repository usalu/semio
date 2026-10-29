import re,sys,collections
G=sys.argv[1]
files=[l.rstrip('\n') for l in open(G)]
SIG=re.compile(r'fn\s+reference_birth\s*\(\s*\)\s*->\s*Option<\s*fn\s*\(\s*\)\s*->\s*([A-Za-z0-9_:<>, ]+?)\s*>\s*\{')
def match_brace(t,i):
    d=0;n=len(t);j=i
    while j<n:
        c=t[j]
        if c=='/' and t[j+1:j+2]=='/':
            j=t.index('\n',j);continue
        if c=='"':
            j+=1
            while t[j]!='"':
                if t[j]=='\\': j+=1
                j+=1
        elif c=="'" :
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[j:])
            if m: j+=len(m.group(0))-1
        elif c=='{': d+=1
        elif c=='}':
            d-=1
            if d==0: return j
        j+=1
    raise Exception("unbalanced")
kinds=collections.Counter()
for f in files:
    t=open(f).read()
    for m in SIG.finditer(t):
        b=m.end()-1
        e=match_brace(t,b)
        body=t[b+1:e].strip()
        k='other'
        if body.startswith('Some(||{') and body.endswith('})'): 
            inner=body[len('Some(||{'):-2]
            k='closure-multiline' if '\n' in inner.strip() else 'closure-oneline'
            if 'return' in inner: k+='-return'
        elif body.startswith('Some(||') and body.endswith(')'): k='closure-expr'
        elif body.startswith('None'): k='none'
        elif 'reference_birth()' in body: k='delegate'
        kinds[k]+=1
        if k not in('closure-oneline','closure-multiline'): print(k,f,body[:200].replace('\n',' '))
print(kinds)
import json
bodies=collections.Counter()
for f in files:
    t=open(f).read()
    for m in SIG.finditer(t):
        b=m.end()-1;e=match_brace(t,b);body=t[b+1:e].strip()
        if body.startswith('Some(||{'):
            inner=body[len('Some(||{'):-2].strip()
            if '\n' in inner or len(inner)>90 or 'expect' in inner or 'unwrap' in inner or 'panic' in inner or '?' in inner: print('NOTE',f[-60:],repr(inner[:300]))
            bodies[re.sub(r'[A-Za-z0-9_]+','x',inner)[:40]]+=1
print(bodies.most_common(15))
