import os,re,sys,subprocess,json
R=os.getcwd()
out=subprocess.run(['bash','-c','grep -rl "semio-plugin-ownership" --include=Cargo.toml --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.git --exclude-dir=.🧬semio --exclude-dir=.cache .'],capture_output=True,text=True).stdout.split('\n')
tomls=[os.path.normpath(x) for x in out if x]
MODS={'store_lifecycle':'L','original_store_genesis':'L','transient_publication':'T','bounded_ownership':'O','sdk_fault':'O','reserved_job_session':'O'}
def walk(f,seen):
    f=os.path.normpath(f)
    if f in seen: return
    if not os.path.exists(f): return
    try: s=open(f,encoding='utf8').read()
    except Exception: return
    seen.add(f); d=os.path.dirname(f)
    stem=os.path.splitext(os.path.basename(f))[0]
    for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',s):
        pm=re.search(r'path\s*=\s*"([^"]+)"',m.group(1)); name=m.group(2)
        if pm: walk(os.path.join(d,pm.group(1)),seen)
        else:
            for c in [os.path.join(d,name+'.rs'),os.path.join(d,name,'mod.rs'),os.path.join(d,stem,name+'.rs'),os.path.join(d,stem,name,'mod.rs')]:
                if os.path.exists(c): walk(c,seen); break
    for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)',s): walk(os.path.join(d,m.group(1)),seen)
res={}
ALL=set()
for t in tomls:
    d=os.path.dirname(t)
    txt=open(t,encoding='utf8').read(); roots=[]
    for sec in re.split(r'(?m)^(?=\[)',txt):
        if re.match(r'\[(lib|\[bin\]|\[test\]|\[example\]|\[bench\])\]?',sec) or re.match(r'\[\[(bin|test|example|bench)\]\]|\[lib\]',sec):
            m=re.search(r'(?m)^path\s*=\s*"([^"]+)"',sec)
            if m: roots.append(m.group(1))
    if os.path.exists(os.path.join(d,'src/lib.rs')): roots.append('src/lib.rs')
    if os.path.exists(os.path.join(d,'src/main.rs')): roots.append('src/main.rs')
    seen=set()
    for r in roots: walk(os.path.join(d,r),seen)
    ALL|=seen
    need=set()
    for f in seen:
        s=open(f,encoding='utf8').read()
        for m in re.finditer(r'semio_plugin_ownership(?:\s*::\s*(\{[^;]*?\}\s*;|\w+))?',s):
            tail=m.group(1) or ''
            if not tail: need.add('O')
            for mod,k in MODS.items():
                if re.search(r'\b'+mod+r'\b',tail if tail.startswith('{') else tail): need.add(k)
    res[t]=sorted(need)
json.dump(res,open('/tmp/need.json','w'),ensure_ascii=False,indent=0)
print(len(tomls),sum(1 for v in res.values() if v))
for k,v in res.items():
    if v: print(v,k[-110:])

hits=subprocess.run(['bash','-c','grep -rlE "semio_plugin_ownership\\s*::\\s*(\\{[^;]*)?(store_lifecycle|original_store_genesis|transient_publication)|(store_lifecycle|original_store_genesis|transient_publication)" --include=*.rs --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.git --exclude-dir=.🧬semio --exclude-dir=.cache . | xargs grep -lE "semio_plugin_ownership"'],capture_output=True,text=True).stdout.split('\n')
orph=[h for h in hits if h and os.path.normpath(h) not in ALL]
print("orphans",len(orph))
for o in orph: print(o)
