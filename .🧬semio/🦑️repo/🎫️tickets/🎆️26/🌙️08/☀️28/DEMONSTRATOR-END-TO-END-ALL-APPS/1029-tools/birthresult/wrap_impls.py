#!/usr/bin/env python3
"""Rewrites every `fn reference_birth()->Option<fn()->T>{Some(||{BODY})}` impl to the typed
`Result<T,semio_framework_plugin::Fault>` hook. Scoped by the exact method name and the exact
`Option<fn()->T>` return type; fail-closed on an empty root or an unrecognised shape.
usage: wrap_impls.py <files-list> <dry|apply>
"""
import re,sys
FILES,MODE=sys.argv[1],sys.argv[2]
files=[l.rstrip('\n') for l in open(FILES,encoding='utf-8') if l.strip()]
if not files: sys.exit("empty root: refusing to run")
SIG=re.compile(r'fn\s+reference_birth\s*\(\s*\)\s*->\s*Option<\s*fn\s*\(\s*\)\s*->\s*([A-Za-z0-9_:<>, ]+?)\s*>\s*\{')
def match_brace(t,i):
    d=0;j=i;n=len(t)
    while j<n:
        c=t[j]
        if c=='/' and t[j+1:j+2]=='/':
            j=t.index('\n',j);continue
        if c=='"':
            j+=1
            while t[j]!='"':
                if t[j]=='\\': j+=1
                j+=1
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[j:])
            if m:j+=len(m.group(0))-1
        elif c=='{':d+=1
        elif c=='}':
            d-=1
            if d==0:return j
        j+=1
    raise SystemExit("unbalanced braces")
DIRTY=re.compile(r'expect\(|unwrap|panic!|\bmatch\b|unreachable')
sigs=wrapped=manual=0;manual_list=[];nocrate=[]
for f in files:
    t=open(f,encoding='utf-8').read()
    edits=[]
    for m in SIG.finditer(t):
        sigs+=1
        T=m.group(1).strip()
        b=m.end()-1;e=match_brace(t,b)
        body=t[b+1:e]
        s=body.strip()
        new_sig=f"fn reference_birth()->Option<fn()->Result<{T},semio_framework_plugin::Fault>>{{"
        mm=re.fullmatch(r'Some\(\|\|\{(.*)\}\)',s,re.S)
        if 'semio_framework_plugin' not in t:nocrate.append(f)
        if not mm: raise SystemExit(f"unrecognised body shape in {f}: {s[:80]}")
        inner=mm.group(1)
        tail=re.fullmatch(r'(.*)\.expect\(("[^"\\]*")\)',inner.strip(),re.S)
        if DIRTY.search(inner) and tail and not DIRTY.search(tail.group(1)) and tail.group(1).count('(')==tail.group(1).count(')') and 'build(' in tail.group(1):
            wrapped+=1
            lead=inner[:len(inner)-len(inner.lstrip())];trail=inner[len(inner.rstrip()):]
            new_body=body.replace(s,"Some(||{"+lead+tail.group(1)+".map_err(|error|semio_framework_plugin::reference_birth_fault("+tail.group(2)+",format_args!(\"{error:?}\")))"+trail+"})",1)
        elif DIRTY.search(inner):
            manual+=1;manual_list.append(f);continue
        else:
            wrapped+=1
            lead=inner[:len(inner)-len(inner.lstrip())];trail=inner[len(inner.rstrip()):]
            new_body=body.replace(s,"Some(||{"+lead+"Ok("+inner.strip()+")"+trail+"})",1)
        edits.append((m.start(),e+1,new_sig+new_body+"}"))
    if edits and MODE=='apply':
        for a,z,txt in sorted(edits,reverse=True):t=t[:a]+txt+t[z:]
        open(f,'w',encoding='utf-8').write(t)
print(f"signatures={sigs} wrapped={wrapped} manual={manual}")
for f in manual_list:print("MANUAL",f)
for f in nocrate:print("NO semio_framework_plugin TOKEN",f)
