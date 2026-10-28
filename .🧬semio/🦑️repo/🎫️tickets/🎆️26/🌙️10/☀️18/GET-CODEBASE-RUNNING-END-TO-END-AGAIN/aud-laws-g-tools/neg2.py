import glob, os, re, sys
C=sys.argv[1]; FE='️'
n=lambda s: s.replace(FE,'')
negrx=re.compile(r"red before|RED before|fails? before|failed before|before the fix|before (?:the )?fix|temporarily|mutant|negative control|verified (?:it )?(?:red|by making)|was red|went red|\bRED\b|control run|reverted", re.I)
ev=[]
for f in sorted(glob.glob(os.path.join(C,'result-*-g.md'))+glob.glob(os.path.join(C,'result-*-f.md'))):
    for line in open(f,encoding='utf-8',errors='replace'):
        if negrx.search(line): ev.append((os.path.basename(f), n(line)))
rows=[r.split('\t') for r in open(os.path.join(C,'🗑️generated/aud-laws-g/scores.tsv'),encoding='utf-8').read().splitlines()[1:]]
res=[]
for r in rows:
    path=n(r[0]); parts=path.split('/')
    idx=[i for i,p in enumerate(parts) if p.endswith('tests') or p=='🧪tests']
    owner=parts[idx[-1]-1] if idx and idx[-1]>0 else parts[-2]
    owner=re.sub(r'^[^\w]+','',owner)  # strip emoji prefix
    key=owner.split('-')[-1] if '-' in owner else owner
    cands=[owner, owner.replace(' ','')]
    hit=set()
    for f,l in ev:
        if len(owner)>=5 and owner in l: hit.add(f)
    res.append((path,len(hit),','.join(sorted(hit))[:90]))
with open(os.path.join(C,'🗑️generated/aud-laws-g/neg.tsv'),'w',encoding='utf-8') as o:
    for p,c,w in res: o.write(f"{p}\t{c}\t{w}\n")
print('evidence', len(ev), 'files with', sum(1 for x in res if x[1]), 'of', len(res))
