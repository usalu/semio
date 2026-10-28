import glob, os, re, sys
C=sys.argv[1]; FE='️'
n=lambda s: s.replace(FE,'')
negrx=re.compile(r"red before|RED before|fails? before|failed before|before the fix|before (?:the )?fix|temporarily|mutant|negative control|verified (?:it )?(?:red|by making)|was red|went red|red \(before|\bRED\b|before:\s*\d|control run|reverted", re.I)
ev=[]
for f in sorted(glob.glob(os.path.join(C,'result-*-g.md'))+glob.glob(os.path.join(C,'result-*-f.md'))):
    for line in open(f,encoding='utf-8',errors='replace'):
        if negrx.search(line):
            ev.append((os.path.basename(f), n(line)))
rows=[r.split('\t') for r in open(os.path.join(C,'🗑️generated/aud-laws-g/scores.tsv'),encoding='utf-8').read().splitlines()[1:]]
out=[]
for r in rows:
    path=n(r[0]); parts=path.split('/')
    keys=set()
    for p in parts[-4:]:
        if len(p)>3: keys.add(p)
    tokens=[parts[-1], parts[-2] if len(parts)>1 else '']
    hits=[(f,l) for f,l in ev if any(k and k in l for k in [parts[-2]] if k) ]
    # file-level: basename match or parent-dir match in a negative-evidence line
    hit=[f for f,l in ev if (parts[-2] in l and len(parts[-2])>4) or (parts[-3] in l and len(parts[-3])>4 and len(parts)>=3)]
    out.append((path, len(set(hit)), ','.join(sorted(set(hit)))[:80]))
with open(os.path.join(C,'🗑️generated/aud-laws-g/neg.tsv'),'w',encoding='utf-8') as o:
    for p,c,w in out: o.write(f"{p}\t{c}\t{w}\n")
print('evidence lines', len(ev), 'files with >=1', sum(1 for _,c,_ in out if c))
