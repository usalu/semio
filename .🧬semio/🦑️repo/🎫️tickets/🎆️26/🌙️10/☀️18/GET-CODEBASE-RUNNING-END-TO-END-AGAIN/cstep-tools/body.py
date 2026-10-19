"""cstep: rewrite a bare InteractiveJobCloseStep-valued fn body into a Result<_, ValueError>-valued body.
transform_body(src, ics) -> (new_body_text, flags)
  - `return E;`            -> `return Ok(E);`
  - tail expression E      -> `Ok(E)`
  - ICS::Refused{kind:e.kind,progress:e.retained_progress()} -> `return Err(e)`
Everything it cannot prove is reported in flags."""
import re,os,sys
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace

def _end_of_expr(m,i):
    d=0;n=len(m);j=i
    while j<n:
        c=m[j]
        if c in '([{': d+=1
        elif c in ')]}':
            if d==0: return j
            d-=1
        elif c==';' and d==0: return j
        elif c==',' and d==0: return j
        j+=1
    return n

def closure_spans(m):
    out=[]
    for c in re.finditer(r'(?:[(,=]|\bmove\b|\breturn\b|=>)\s*(\|[^|\n{}]*\||\|\|)',m):
        i=c.end();n=len(m)
        while i<n and m[i] in ' \t\r\n': i+=1
        if m.startswith('->',i):
            j=m.find('{',i)
            if j<0: continue
            i=j
        if i<n and m[i]=='{':
            e=match_brace(m,i)
        else:
            e=_end_of_expr(m,i)
        out.append((c.start(1),e))
    return out

def _stmt_split(m):
    """top-level statement spans (start,end,has_semi) in masked block text m (no outer braces)."""
    out=[];n=len(m);i=0;d=0;start=0
    blocky=re.compile(r'\s*(?:if|match|loop|while|for|unsafe)\b|\s*\{|\s*macro_rules\s*!|\s*\'\w+\s*:')
    j=0
    while j<n:
        c=m[j]
        if c in '([{': d+=1
        elif c in ')]}': d-=1
        if d==0 and c==';':
            out.append((start,j+1,True));start=j+1
        elif d==0 and c=='}':
            seg=m[start:j+1]
            if blocky.match(seg):
                k=j+1
                while k<n and m[k] in ' \t\r\n': k+=1
                nxt=m[k:k+4]
                if nxt.startswith('else') or (k<n and m[k] in '.?'):
                    j+=1;continue
                out.append((start,j+1,False));start=j+1
        j+=1
    if m[start:].strip(): out.append((start,n,False))
    return out

def transform_body(src,ics,refused_ok=True):
    flags=[]
    names='|'.join(re.escape(x) for x in sorted(ics,key=len,reverse=True))
    m=mask(src)
    helper=re.compile(r'[ \t]*\blet\s+(?P<name>refused|refuse)\s*=\s*\|\s*(?P<p>\w+)\s*(?::[^|]*)?\|\s*(?:[\w]+::)*(?:'+names+r')::Refused\s*\{\s*kind\s*:\s*(?P=p)\.kind\s*,\s*progress\s*:\s*(?P=p)\.retained_progress\(\)\s*\}\s*;[ \t]*\n?')
    hnames=set()
    for h in reversed(list(helper.finditer(m))):
        hnames.add(h.group('name'))
        src=src[:h.start()]+src[h.end():]
        m=mask(src)
    for name in hnames:
        for c in reversed(list(re.finditer(r'(?P<ret>\breturn\s+)?(?<![\w.])'+name+r'\s*\(',m))):
            a=c.start('ret') if c.group('ret') else c.start()
            src=src[:a]+'return Err('+src[c.end():]
            m=mask(src)
        if re.search(r'(?<![\w.])'+name+r'\b(?!\s*\()',m): flags.append('helper-used-as-value:'+name)
    refpat=re.compile(r'(?P<ret>\breturn\s+)?(?:[\w]+::)*(?:'+names+r')::Refused\s*\{\s*kind\s*:\s*(?P<e>\w+)\.kind\s*,\s*progress\s*:\s*(?P=e)\.retained_progress\(\)\s*\}')
    cl=closure_spans(m)
    edits=[]
    for r in refpat.finditer(m):
        edits.append((r.start(),r.end(),'return Err(%s)'%r.group('e')))
        if any(a<=r.start()<b for a,b in cl): flags.append('refused-in-closure')
    kindpat=re.compile(r'(?P<ret>\breturn\s+)?(?:[\w]+::)*(?:'+names+r')::Refused\s*\{\s*kind\s*:\s*(?P<k>(?:[\w]+::)*ValueRefusalKind::\w+)\s*,\s*progress\s*:\s*(?:[\w]+::)*(?:RetainedCloneProgress::default\(\)|Default::default\(\))\s*\}')
    for r in kindpat.finditer(m):
        kind=r.group('k');pre='' if '::' in kind.replace('ValueRefusalKind::','',1) or kind.count('::')>1 else ''
        vk=kind if kind.count('::')>1 else 'semio_framework_value::'+kind
        text={'DepthLimit':'interactive job close exceeds its admitted depth'}.get(kind.split('::')[-1],'interactive job close refused')
        edits.append((r.start(),r.end(),'return Err(semio_framework_value::ValueError::literal(%s, "%s"))'%(vk,text)))
        if any(a<=r.start()<b for a,b in cl): flags.append('refused-in-closure')
    covered=[(a,b) for a,b,_ in edits]
    left=[r for r in re.finditer(r'\bRefused\b',m) if not any(a<=r.start()<b for a,b in covered)]
    if left: flags.append('refused-shape:%d'%len(left))
    if re.search(r'(?<![\w])fn\s+\w+',m): flags.append('nested-fn')
    for r in re.finditer(r'\breturn\b',m):
        if any(a<=r.start()<b for a,b in covered): continue
        if any(a<=r.start()<b for a,b in cl): continue
        s0=r.end();e=_end_of_expr(m,s0)
        expr=m[s0:e]
        if not expr.strip(): flags.append('bare-return');continue
        s0+=len(expr)-len(expr.lstrip());expr=m[s0:e]
        e=s0+len(expr.rstrip())
        if expr.startswith('Err('): continue
        edits.append((s0,e,None,'wrap'))
    stmts=_stmt_split(m)
    if stmts:
        a,b,semi=stmts[-1]
        if not semi:
            seg=m[a:b]
            if re.match(r'\s*return\b',seg): pass
            elif any(x<=a<y for x,y in covered): pass
            else:
                edits.append((a+len(seg)-len(seg.lstrip()),a+len(seg.rstrip()),None,'wrap'))
        else:
            tail_seg=m[a:b].strip()
            if not re.match(r'return\b',tail_seg) and not tail_seg.startswith('panic') and not tail_seg.startswith('unreachable'):
                flags.append('ends-with-semicolon')
    norm=[]
    for e in edits:
        if len(e)==3: norm.append(e)
        else: norm.append((e[0],e[1],'Ok('))
    spans=sorted(norm,key=lambda t:(t[0],-t[1]))
    def apply(lo,hi,items):
        res=[];pos=lo;i=0
        while i<len(items):
            a,b,txt=items[i]
            if a<pos: i+=1;continue
            res.append(src[pos:a])
            inner=[x for x in items[i+1:] if x[0]>=a and x[1]<=b and (x[0],x[1])!=(a,b)]
            if txt=='Ok(':
                body=apply(a,b,inner)
                res.append('Ok(%s)'%body.strip())
            else:
                res.append(txt)
            pos=b
            i+=1
            while i<len(items) and items[i][0]<b: i+=1
        res.append(src[pos:hi])
        return ''.join(res)
    return apply(0,len(src),spans),flags
