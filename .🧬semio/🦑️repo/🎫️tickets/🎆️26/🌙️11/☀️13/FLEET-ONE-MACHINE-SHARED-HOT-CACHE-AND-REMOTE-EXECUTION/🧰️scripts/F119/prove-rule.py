#!/usr/bin/env python3
"""Read-only: tests the candidate path-to-$id rule against every module that has a conforming root $id.
Usage: bun dump-modules.ts <dump.json> ; python3 prove-rule.py <dump.json>. Never writes to the repo; refuses an empty path."""
import json,re,collections,sys
if len(sys.argv)<2 or not sys.argv[1]: sys.exit("refusing: empty dump path")
d=json.load(open(sys.argv[1]))
B="https://json.schemas.assets.semio-tech.com/"
SC=re.compile(r'^[a-z0-9]+(?:-[a-z0-9]+)*(?:\.[a-z0-9]+(?:-[a-z0-9]+)*)*$'); FC=re.compile(r'^[a-z0-9]+(?:-[a-z0-9]+)*\.json$')
def conf(i):
    if not i or not i.startswith(B): return None
    s=i[len(B):].split('/'); f=s.pop()
    return (s,f) if s and FC.match(f) and SC.match('.'.join(s)) else None
strip=lambda s: re.sub(r'^[^a-z0-9]+','',s)
def rule(path):
    p=path.split('/')[:-1]
    if p[0]=='✏️s' and len(p)>8 and p[3]=='🗿️artifacts':
        out=['s',strip(p[2]),strip(p[4])]
        if strip(p[8])!='any': out+=[strip(p[6]),strip(p[8])]
        out+=['mutation' if x=='🧬️mutations' else strip(x) for x in p[9:] if x!='🧬️schema']
        return [o for o in out if o not in('editor','viewer','modes')]
    if p[:2]==['🧰️framework','🛍️products']: out=p[2:]
    elif p[:2]==['🧰️framework','🔨️modules']: out=['framework']+p[2:]
    else: out=p
    return [('mutation' if s=='🧬️mutations' else strip(s)) for s in out if s not in('🔨️modules','🧬️schema')]
def area(path):
    p=path.split('/')
    if p[0]=='✏️s': return 'plugins' if p[1].startswith('🔌') else 'impl-other'
    return p[1]+'/'+(p[2] if p[1].startswith('🛍') else '') if p[0]=='🧰️framework' else p[0]
H=[h for h in d['have'] if h['id']]
C=[h for h in H if conf(h['id'])]
print(f"roots with an id: {len(H)}, conforming: {len(C)}, non-conforming: {len(H)-len(C)}")
st=collections.defaultdict(collections.Counter); cls=collections.Counter()
for h in C:
    a=area(h['path']); s,f=conf(h['id']); r=rule(h['path']); st[a]['n']+=1
    st[a]['scope']+= s==r; st[a]['exact']+= s==r and f=='schema.json'
    if s==r: cls['scope ok, facet filename differs' if f!='schema.json' else 'exact']+=1
    elif 'mutation' in s and 'mutation' in r and s[:-1]!=r[:-1] and s[:len(r)-1]==r[:-1]: cls['mutation verb renamed (descriptor name)']+=1
    elif s[0]=='app' or s[0]=='window': cls['app/window lane scheme']+=1
    else: cls['hand-named or restructured scope']+=1
tot=sum(v['n'] for v in st.values()); ex=sum(v['exact'] for v in st.values()); sc=sum(v['scope'] for v in st.values())
for a,v in sorted(st.items()): print(f"{a}: n={v['n']} scope={v['scope']} exact={v['exact']}")
print(f"TOTAL exact {ex}/{tot} = {100*ex/tot:.1f}%, scope-only {sc}/{tot} = {100*sc/tot:.1f}%")
print(dict(cls))
