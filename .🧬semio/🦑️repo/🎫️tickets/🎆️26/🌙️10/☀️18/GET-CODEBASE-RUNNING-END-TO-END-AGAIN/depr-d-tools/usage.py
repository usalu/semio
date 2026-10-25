import json,subprocess,sys,re,collections
G=sys.argv[1]
d=json.load(open(G+'/new.json'))
names=set(e['name'] for e in d['newDeps'] if e['ecosystem']=='js')
files=subprocess.run("git ls-files -c -o --exclude-standard -- '*.ts' '*.tsx' '*.js' '*.mjs' '*.mts' '*.cjs' '*.jsx'",shell=True,capture_output=True,text=True).stdout.split('\n')
rx=re.compile(r'''(?:from\s+|import\s*\(\s*|require\s*\(\s*|import\s+|export\s+\*\s+from\s+)["']((?:@[^/"']+/)?[^/"'.][^/"']*)''')
out=collections.defaultdict(list)
for f in files:
    if not f or '.🧬semio/' in f or f.startswith(('node_modules','temp/')): continue
    try: s=open(f,encoding='utf-8',errors='ignore').read()
    except Exception: continue
    seen=set()
    for m in rx.finditer(s):
        n=m.group(1)
        if n in names and n not in seen:
            seen.add(n); out[n].append(f)
json.dump(out,open(G+'/usage.json','w'),indent=1,ensure_ascii=False)
for n in sorted(names): print(n,len(out.get(n,[])))
