import os,re,sys,json
root=os.getcwd()
files=[l.rstrip('\n') for l in open(sys.argv[1])]
out={}
for f in files:
    f=os.path.normpath(f)
    src=open(f,encoding='utf-8').read()
    dec='fn validate_sqlite_snapshot_subset_decoding' in src
    enc='fn validate_sqlite_snapshot_subset_encoding' in src
    sub='fn validate_sqlite_snapshot_subset(' in src
    d=os.path.dirname(f); pkg=None; man=None
    while d and d!=root and d!='/':
      for m in (os.path.join(d,'Cargo.toml'),os.path.join(d,'📦️packages','🦀️rust','Cargo.toml')):
        if os.path.exists(m) and pkg is None:
            t=open(m,encoding='utf-8').read()
            mm=re.search(r'^\[package\]\s*(?:^[^\[\n]*\n)*?^name\s*=\s*"([^"]+)"',t,re.M)
            if mm: pkg=mm.group(1); man=m
      if pkg: break
      d=os.path.dirname(d)
    out[f]=dict(dec=dec,enc=enc,sub=sub,pkg=pkg,man=man)
json.dump(out,open(sys.argv[2],'w'),ensure_ascii=False,indent=1)
aff=[(v['pkg'],f,v) for f,v in out.items() if not (v['dec'] and v['enc'])]
print(len(out),'impls',len(aff),'affected (default decoding or encoding)')
from collections import defaultdict
g=defaultdict(list)
for p,f,v in aff:g[p].append((f,v))
for p in sorted(g,key=str):print(p,len(g[p]),[ ('D' if not v['dec'] else '')+('E' if not v['enc'] else '')+('S' if v['sub'] else '') for f,v in g[p]])
