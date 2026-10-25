import json,subprocess,sys,re,collections
G=sys.argv[1]
d=json.load(open(G+'/new.json'))
names=sorted(set(e['name'] for e in d['newDeps'] if e['ecosystem']=='js'))
files=subprocess.run("git ls-files -c -o --exclude-standard -- '*.ts' '*.tsx' '*.js' '*.mjs' '*.mts' '*.cjs' '*.jsx' '*.json' '*.py' '*.sh' '*.rs'",shell=True,capture_output=True,text=True).stdout.split('\n')
alt='|'.join(re.escape(n) for n in names)
rx=re.compile(r'''["'](%s)(?:/[^"']*)?["']'''%alt)
out=collections.defaultdict(list)
for f in files:
    if not f or '.🧬semio/' in f or f.startswith(('node_modules','temp/')) or f.endswith(('package.json','bun.lock','🔒️dependencies.json','build-owners.json')): continue
    try: s=open(f,encoding='utf-8',errors='ignore').read()
    except Exception: continue
    seen=set()
    for m in rx.finditer(s):
        n=m.group(1)
        if n not in seen: seen.add(n); out[n].append(f)
json.dump(out,open(G+'/usage2.json','w'),indent=1,ensure_ascii=False)
test=lambda f: '/🧪️' in '/'+f or '/🔬️probes/' in f or '/🔮️oracles/' in f or '/🏭️generator/' in f or '/🧫️fixtures/' in f
for n in names:
    fs=out.get(n,[])
    nt=[f for f in fs if not test(f)]
    print(n,len(fs),'NONTEST:',len(nt))
    for f in nt[:8]: print('     ',f[-190:])
