import re,sys,collections,os,subprocess
G=sys.argv[1]
root='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets'
files=subprocess.check_output(['git','ls-files','*.rs'],cwd=root,text=True).split('\n')
files=[f for f in files if f and '🧪️' not in f]
idx=collections.defaultdict(list)
pat=re.compile(r'^\s*(pub(?:\([^)]*\))?\s+)?(?:const\s+fn|async\s+fn|unsafe\s+fn|fn|const|static|struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)')
for f in files:
    for n,l in enumerate(open(os.path.join(root,f),encoding='utf8'),1):
        m=pat.match(l)
        if m: idx[m.group(2)].append((f,n,(m.group(1) or 'priv').strip()))
names=set()
for l in open(G+'/errs.txt'):
    m=re.search(r'cannot find (\w+) `([^`]+)`',l)
    if m: names.add(m.group(2))
for n in sorted(names):
    print(n, '=>', '; '.join(f'{f.split("/")[-2] if f.count("/")>1 else f}:{ln}:{v}' for f,ln,v in idx.get(n,[])) or 'NONE')
