#!/usr/bin/env python3
import json,glob,os,re,subprocess,sys,collections
R='/Users/Shared/semio/'
LIB='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
G=R+'.cache/b/g/m/generations/'
SPEC=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+|require\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
def fresh_files(lo=218,hi=223):
    d={}
    for f in glob.glob(G+'m/*.json'):
        x=json.load(open(f))
        if x['implementation'].startswith('b1f7ad'): d[x['generation']]=x
    owners={}
    for g in range(lo,hi+1):
        for k in d[g]['built']['dirty']: owners[k]=d[g]['owners'][k]['fragment']
    held=collections.defaultdict(set); toks={}
    for k,h in owners.items():
        fr=json.loads(subprocess.run(['brotli','-dc',G+f'o/{h}.json.br'],capture_output=True).stdout)
        for g,l in (fr.get('node',{}).get('namedInputs') or {}).items():
            if not (g.startswith('commandSources') or g=='testCommandSources'): continue
            for x in l:
                if isinstance(x,dict) and x.get('env','').startswith('SEMIO_SOURCE_CLOSURE_'): toks.setdefault(x['env'][21:],set()).add(k)
    for t,os_ in toks.items():
        g=glob.glob(G+'../build-input-closures/'+t+'*.json')
        if g:
            for f in json.load(open(g[0]))['files']: held[f]|=os_
    return held,len(owners)
def edges(files):
    out={}
    fs=set(files)
    for f in files:
        if not re.search(r'\.(ts|mjs|js|cjs|mts|tsx)$',f): continue
        try: src=open(R+f,encoding='utf8').read()
        except Exception: continue
        dst=set()
        for m in SPEC.finditer(src):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            for c in (p,p[:-3]+'.ts' if p.endswith('.js') else p,p.replace('.mjs','.d.mts')):
                if c in fs: dst.add(c); break
        out[f]=dst
    return out
if __name__=='__main__':
    held,n=fresh_files()
    cands=sys.argv[1:]
    E=edges(list(held))
    print('fresh owners',n,'closure files',len(held))
    for c in cands:
        c=LIB+c
        inside=[f for f in held if f.startswith(c)]
        holders=set().union(*[held[f] for f in inside]) if inside else set()
        ports=collections.Counter()
        for f,ds in E.items():
            if f.startswith(c): continue
            for t in ds:
                if t.startswith(c): ports[t]+=1
        print(f'{c[len(LIB):]}: files {len(inside)} holders {len(holders)}/{n} ports {len(ports)}')
        for p,k in ports.most_common(8): print('    port',p[len(LIB):] if p.startswith(LIB) else p,k)
