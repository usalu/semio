import os,re,sys
root=sys.argv[1]
paths=set();files=[]
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in('node_modules','target','.git')]
    for f in fs:
        p=os.path.join(d,f)
        if f.endswith('.rs'):
            files.append(p)
            try:s=open(p,encoding='utf-8').read()
            except:continue
            for m in re.finditer(r'include!\("([^"]+)"\)',s):paths.add(os.path.normpath(os.path.join(d,m.group(1))))
            for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',s):paths.add(os.path.normpath(os.path.join(d,m.group(1))))
        elif f=='Cargo.toml':
            s=open(p,encoding='utf-8').read()
            for m in re.finditer(r'^\s*path\s*=\s*"([^"]+)"',s,re.M):paths.add(os.path.normpath(os.path.join(d,m.group(1))))
for p in sorted(files):
    if '/🧪️tests/' in p and not any(x in p for x in('/👽️guest/','/🔮️oracles/','/🏭️bridge/')):
        if os.path.normpath(p) not in paths and not p.endswith('/build.rs'):
            print(p)
