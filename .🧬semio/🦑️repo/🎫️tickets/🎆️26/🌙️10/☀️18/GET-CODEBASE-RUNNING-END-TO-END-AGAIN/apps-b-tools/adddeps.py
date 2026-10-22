#!/usr/bin/env python3
"""Adds the crates a derive expands to as path dependencies of every crate whose module tree uses it. usage: adddeps.py <derive-regex> <depname> [--dry]"""
import os,re,sys
mine=['➗️mathematical','🗒️note','📕️norm','📖️playbook','🌀️procedural','🏭️process','🧩️puzzle','🖨️raster','💡️reasoning','📸️remodel','🎬️sequence','🎥️shooting','🪵️sourcing','🪐️space','🗄️stdio','🔱️trinity','🌿️vcs','🀄️wfc','✒️writer']
ROOT='/Users/ueli/Documents/semio'
os.chdir(ROOT)
pat=re.compile(sys.argv[1]); dep=sys.argv[2]; dry='--dry' in sys.argv
FW={}
for r,ds,fs in os.walk('🧰️framework'):
    ds[:]=[d for d in ds if d not in('node_modules','target')]
    if 'Cargo.toml' in fs:
        t=open(os.path.join(r,'Cargo.toml'),encoding='utf-8').read()
        m=re.search(r'^name = "([^"]+)"',t,re.M)
        if m and m.group(1)==dep: FW[dep]=r
assert dep in FW,'framework crate not found'
crates=[]  # (libdir, manifestdir)
for p in mine:
    for r,ds,fs in os.walk('✏️s/🔌️plugins/'+p):
        ds[:]=[d for d in ds if d not in('node_modules','target')]
        if 'Cargo.toml' in fs:
            t=open(os.path.join(r,'Cargo.toml'),encoding='utf-8').read()
            if '[package]' not in t: continue
            m=re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"',t,re.M|re.S)
            lib=os.path.normpath(os.path.join(r,m.group(1))) if m else os.path.join(r,'src/lib.rs')
            crates.append((os.path.dirname(lib),r,t))
def mounted(libfile):
    seen=set();stack=[libfile]
    while stack:
        fp=stack.pop()
        fp=os.path.realpath(fp)
        if fp in seen or not os.path.isfile(fp): continue
        seen.add(fp)
        try:s=open(fp,encoding='utf-8').read()
        except:continue
        d=os.path.dirname(fp)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',s):
            stack.append(os.path.join(d,m.group(1)))
        for m in re.finditer(r'include!\(\s*"([^"]+)"',s):
            stack.append(os.path.join(d,m.group(1)))
    return seen
owner={}
for ld,mr,t in crates:
    m=re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"',t,re.M|re.S)
    lib=os.path.normpath(os.path.join(mr,m.group(1))) if m else os.path.join(mr,'src/lib.rs')
    for fp in mounted(lib): owner.setdefault(fp,[]).append((mr,t))
targets={}
for p in mine:
    for r,ds,fs in os.walk('✏️s/🔌️plugins/'+p):
        ds[:]=[d for d in ds if d not in('node_modules','target')]
        for f in fs:
            if not f.endswith('.rs'): continue
            fp=os.path.join(r,f)
            if '/🧪️tests/' in fp: continue
            try:s=open(fp,encoding='utf-8').read()
            except:continue
            if pat.search(s):
                own=owner.get(os.path.realpath(fp),[])
                for mr,t in own: targets[mr]=t
                best=None if not own else (None,)
                for ld,mr,t in ([] if own else crates):
                    d=os.path.dirname(fp)
                    if d==ld or d.startswith(ld+'/'):
                        if best is None or len(ld)>len(best[0]): best=(ld,mr,t)
                if best and best[0]: targets[best[1]]=best[2]
                elif not own: print('no crate for',fp)
n=0
for mr,t in sorted(targets.items()):
    if re.search(r'^'+re.escape(dep)+r'\s*=',t,re.M): continue
    rel=os.path.relpath(FW[dep],mr)
    line=f'{dep} = {{ path = "{rel}" }}'
    lines=t.split('\n')
    # find [dependencies]
    try: i=lines.index('[dependencies]')
    except ValueError:
        lines+=['','[dependencies]',line]
    else:
        j=i+1
        while j<len(lines) and not lines[j].startswith('['):
            j+=1
        k=j
        while k>i+1 and lines[k-1].strip()=='' : k-=1
        # sorted insert
        ins=k
        for q in range(i+1,k):
            nm=lines[q].split('=')[0].strip()
            if nm and not nm.startswith('#') and nm>dep: ins=q;break
        lines.insert(ins,line)
    n+=1
    if not dry: open(os.path.join(mr,'Cargo.toml'),'w',encoding='utf-8').write('\n'.join(lines))
print('crates updated',n,'of',len(targets))
