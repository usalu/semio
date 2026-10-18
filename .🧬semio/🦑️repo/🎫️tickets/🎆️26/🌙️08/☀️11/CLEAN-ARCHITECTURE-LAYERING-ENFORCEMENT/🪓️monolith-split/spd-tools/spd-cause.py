import json,sys,re,os,collections
ROOT='/Users/ueli/Documents/semio'
EXPORT=re.compile(r'^[A-Z][A-Za-z0-9]*$')
def nullable(n):
    t=n.get('type')
    if isinstance(t,list) and 'null' in t:
        r=[k for k in t if k!='null']
        if len(r)==1:
            m={k:v for k,v in n.items() if k!='type'}; m['type']=r[0]; return m
        return None
    for k in ('anyOf','oneOf'):
        a=n.get(k)
        if isinstance(a,list) and len(a)==2 and sum(1 for x in a if isinstance(x,dict) and x.get('type')=='null')==1:
            o=[x for x in a if not(isinstance(x,dict) and x.get('type')=='null')][0]
            return o if isinstance(o,dict) else None
    return None
def shape(n):
    if not isinstance(n,dict): return 'non-dict'
    if '$ref' in n: return 'ref-nonexport'
    if not n: return 'empty-schema'
    if 'enum' in n: return 'enum-nonstring' if not all(isinstance(e,str) for e in n['enum']) else 'enum'
    if 'const' in n and 'type' not in n: return 'const'
    for k in ('oneOf','anyOf'):
        if k in n:
            alts=n[k]
            tagged=all(isinstance(a,dict) and isinstance(a.get('properties'),dict) for a in alts)
            return k+(':objects' if tagged else ':mixed')+'[%d]'%len(alts)
    if 'allOf' in n: return 'allOf'
    t=n.get('type')
    if t is None: return 'no-type'
    if isinstance(t,list): return 'type-multi:'+','.join(map(str,t))
    if t=='array':
        it=n.get('items')
        if isinstance(it,list): return 'tuple'
        if not isinstance(it,dict): return 'array-noitems'
        s=shape(it)
        return 'array<'+s+'>'
    if t=='object':
        if n.get('properties'): return 'object-inline'
        if isinstance(n.get('additionalProperties'),dict) and n['additionalProperties']: return 'map'
        return 'object-open'
    return 'type:'+str(t)
def oe(n):
    return n.get('type')=='object' and isinstance(n.get('properties'),dict) and len(n['properties'])>0
def isref(n):
    return str(n['$ref']).split('/')[-1]
def item(n):
    it=n.get('items'); return it if isinstance(it,dict) else {}
def gql(n):
    if '$ref' in n: return None if EXPORT.match(isref(n)) else 'ref-nonexport'
    p=nullable(n)
    if p is not None:
        r=gql(p); return None if r is None else 'nullable:'+r
    t=n.get('type')
    if t in('string','boolean','integer','number'): return None
    if t=='array':
        r=gql(item(n)); return None if r is None else 'array<'+r+'>'
    return shape(n)
def prt(n):
    if '$ref' in n: return None if EXPORT.match(isref(n)) else 'ref-nonexport'
    p=nullable(n)
    if p is not None:
        r=prt(p)
        if r is None and p.get('type')=='array': return 'nullable:array'
        return None if r is None else 'nullable:'+r
    t=n.get('type')
    if t in('string','boolean','integer','number'): return None
    if t=='array':
        if nullable(item(n)) is not None: return 'array<nullable>'
        r=prt(item(n))
        if r is None and item(n).get('type')=='array': return 'array<array>'
        return None if r is None else 'array<'+r+'>'
    return shape(n)
def tst(n):
    if '$ref' in n: return None if EXPORT.match(isref(n)) else 'ref-nonexport'
    p=nullable(n)
    if p is not None:
        r=tst(p); return None if r is None else 'nullable:'+r
    if 'enum' in n and all(isinstance(e,str) for e in n['enum']): return None
    if 'const' in n and isinstance(n['const'],(str,int,float,bool)): return None
    t=n.get('type')
    if t in('string','boolean','integer','number'): return None
    if t=='array':
        r=tst(item(n)); return None if r is None else 'array<'+r+'>'
    if t=='object' and not n.get('properties'): return None
    return shape(n)
def sig(n, fmt, top=False):
    if fmt=='graphql':
        if 'enum' in n and all(isinstance(e,str) for e in n['enum']): return None
        if oe(n):
            for k,v in n['properties'].items():
                r=gql(v)
                if r: return 'prop:'+r
            return None
        if n.get('type') in('string','integer','number','boolean') or 'oneOf' in n or 'anyOf' in n or (n.get('type')=='object' and not n.get('properties')): return None
        return shape(n)
    if fmt=='proto':
        if oe(n):
            for k,v in n['properties'].items():
                r=prt(v)
                if r: return 'prop:'+r
            return None
        r=prt(n)
        if r is None and n.get('type')!='array': return None
        if 'enum' in n and all(isinstance(e,str) for e in n['enum']): return None
        return r or 'array-root'
    if fmt=='ts':
        if oe(n):
            for k,v in n['properties'].items():
                r=tst(v)
                if r: return 'prop:'+r
            return None
        return tst(n)
def parsig(n, top=False):
    if not isinstance(n,dict): return 'non-dict'
    if '$ref' in n: 
        return None
    p=nullable(n)
    if p is not None:
        s=parsig(p); return ('nullable:'+s) if s else None
    if 'enum' in n and all(isinstance(e,str) for e in n['enum']): return None
    if 'const' in n and isinstance(n['const'],(str,int,float,bool)): return None
    for k in ('oneOf','anyOf'):
        if k in n:
            alts=n[k]
            if alts and all(isinstance(a,dict) and isinstance(a.get('properties'),dict) for a in alts):
                keys=list(alts[0]['properties'].keys())
                for key in keys:
                    tags=[a['properties'].get(key,{}).get('const') for a in alts]
                    if all(isinstance(t,str) for t in tags) and len(set(tags))==len(tags):
                        for a in alts:
                            for kk,v in a['properties'].items():
                                if kk==key: continue
                                s=parsig(v)
                                if s: return 'tagged:prop:'+s
                            if 'allOf' in a: return 'tagged:allOf'
                            if a.get('additionalProperties') is False and 'patternProperties' in a: return 'tagged:pattern'
                        return None
                if all(a.get('type')=='object' and len(a['properties'])==1 for a in alts):
                    for a in alts:
                        s=parsig(list(a['properties'].values())[0])
                        if s: return 'external:'+s
                    return None
                return k+':objects-untagged[%d]'%len(alts)
            return k+':mixed[%d]'%len(alts)
    if 'allOf' in n:
        if len(n['allOf'])==1 and 'properties' not in n: return parsig(n['allOf'][0])
        return 'allOf'
    t=n.get('type')
    if t is None: return 'empty-schema' if not n else 'no-type'
    if isinstance(t,list): return 'type-multi:'+','.join(map(str,t))
    if t in('string','boolean','integer','number'): return None
    if t=='array':
        it=n.get('items')
        if isinstance(it,list):
            for m in it:
                s=parsig(m)
                if s: return 'tuple:'+s
            return None
        if isinstance(it,dict) and it:
            s=parsig(it)
            return ('array<'+s+'>') if s else None
        return None
    if t=='object':
        props=n.get('properties')
        if props:
            if 'patternProperties' in n and n.get('additionalProperties') is False: return 'closed+pattern'
            for k,v in props.items():
                if re.fullmatch(r'0|[1-9][0-9]*',k): return 'index-key'
                s=parsig(v)
                if s: return 'prop:'+s
            return None
        ap=n.get('additionalProperties')
        if isinstance(ap,dict) and ap:
            s=parsig(ap); return ('map<'+s+'>') if s else None
        return None
    return 'type:'+str(t)
def exports(doc):
    out={}
    t=doc.get('title')
    if isinstance(t,str) and (doc.get('type')=='object' or ('type' not in doc and isinstance(doc.get('properties'),dict))) and EXPORT.match(t): out[t]=doc
    for k,v in (doc.get('$defs') or {}).items():
        if EXPORT.match(k) and k not in out: out[k]=v if isinstance(v,dict) else {}
    return out
def main():
    d=json.load(open(sys.argv[1]))
    rows=d['refused']
    out=[]
    cache={}
    for r in rows:
        parts=r.split(' ',2)
        fmt,name,leaf=parts[0],parts[1],parts[2]
        if fmt=='parser': fmt2='parser'
        elif 'typescript' in fmt: fmt2='ts'
        elif 'graphql' in fmt: fmt2='graphql'
        elif 'protobuf' in fmt: fmt2='proto'
        else: fmt2=fmt
        if fmt2 in ('parser',) and not parts[2].endswith('.ts'):
            out.append((fmt2,name,leaf,'dangling-or-needs:'+parts[2][:80])); continue
        directory=os.path.dirname(leaf)
        docp=os.path.join(ROOT,directory,'🔣️.json')
        if docp not in cache:
            try: cache[docp]=exports(json.load(open(docp)))
            except Exception as e: cache[docp]={}
        ex=cache[docp]
        node=ex.get(name)
        if node is None:
            out.append((fmt2,name,leaf,'export-not-found')); continue
        s=parsig(node,True) if fmt2=='parser' else sig(node,fmt2,True)
        out.append((fmt2,name,leaf,s or 'renders?'))
    json.dump(out,open(sys.argv[2],'w'),ensure_ascii=False)
    c=collections.Counter((f,s) for f,n,l,s in out)
    for (f,s),v in sorted(c.items(),key=lambda x:-x[1])[:int(sys.argv[3]) if len(sys.argv)>3 else 60]: print(v,f,s)
main()
