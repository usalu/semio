import os,re,sys
root=sys.argv[1]
names={};tomls=[]
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in('target','node_modules','.git','.cache','🗑️generated')]
    if 'Cargo.toml' in fs:
        p=os.path.join(d,'Cargo.toml');s=open(p,encoding='utf8').read();tomls.append((p,s))
        m=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',s,re.M|re.S)
        if m: names[os.path.realpath(d)]=m.group(1)
for p,s in tomls:
    for m in re.finditer(r'^([A-Za-z0-9_\-]+)\s*=\s*\{([^}\n]*\bpath\s*=\s*"([^"]+)"[^}\n]*)\}',s,re.M):
        k,body,path=m.groups()
        pk=re.search(r'package\s*=\s*"([^"]+)"',body)
        want=pk.group(1) if pk else k
        tp=os.path.realpath(os.path.join(os.path.dirname(p),path))
        have=names.get(tp)
        if have is None: print('MISSING',p,k,path)
        elif have!=want: print('MISMATCH',p,k,'->',have)
