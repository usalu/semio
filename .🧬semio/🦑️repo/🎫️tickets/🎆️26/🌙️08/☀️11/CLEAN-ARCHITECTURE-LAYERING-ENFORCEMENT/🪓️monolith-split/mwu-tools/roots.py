import os,re,json,subprocess
R='/Users/ueli/Documents/semio'
out=subprocess.run(['git','-c','core.quotepath=false','-c','core.fsmonitor=false','ls-files','-z','--','✏️s'],cwd=R,capture_output=True,text=True).stdout.split('\0')
tomls=[f for f in out if f.endswith('/📦️packages/🦀️rust/Cargo.toml') or f.endswith('/Cargo.toml')]
rows=[]
for t in tomls:
    p=os.path.join(R,t)
    if not os.path.exists(p): continue
    txt=open(p,encoding='utf8').read()
    m=re.search(r'^name\s*=\s*"([^"]+)"',txt,re.M)
    if not m: continue
    lib=re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"',txt,re.M|re.S)
    d=os.path.dirname(p)
    libp=os.path.join(d,lib.group(1)) if lib else os.path.join(d,'src/lib.rs')
    if not os.path.exists(libp): continue
    lt=open(libp,encoding='utf8').read()
    mir=os.path.join(d,'🪞️mirror/🦀️.rs')
    mt=open(mir,encoding='utf8').read() if os.path.exists(mir) else ''
    def cnt(s): return len(re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?semio_[a-z0-9_]+::',s,re.M))
    rows.append(dict(toml=os.path.relpath(d,R),name=m.group(1),lib=os.path.relpath(libp,R),libext=cnt(lt),mir=bool(mt),mirext=cnt(mt),liblines=lt.count('\n'),mirlines=mt.count('\n')))
json.dump(rows,open('/Users/ueli/.cache/mwu/roots.json','w'),ensure_ascii=False)
print(len(rows))
a=[r for r in rows if r['mirext']>0]; b=[r for r in rows if r['libext']>0 and r['mirext']==0]; c=[r for r in rows if r['libext']>0]
print('mirror crates',len(a),'lib-only ext',len(b),'libext any',len(c))
import collections
print(sum(r['mirext'] for r in rows),sum(r['libext'] for r in rows))
