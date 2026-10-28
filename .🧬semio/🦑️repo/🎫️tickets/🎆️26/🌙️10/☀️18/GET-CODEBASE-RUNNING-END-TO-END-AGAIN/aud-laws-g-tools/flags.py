import re, sys, os
REPO='/Users/ueli/Documents/semio'
FE='️'
rx = {
 'skip': re.compile(r"\.skip\(|skipIf|\bxit\(|\bxdescribe\(|#\[ignore|pytest\.mark\.skip|\bit\.todo"),
 'env': re.compile(r"process\.env\.[A-Z_]+|os\.environ|env::var|SEMIO_[A-Z_]+"),
 'taut': re.compile(r"expect\((\w[\w.]*)\)\.(?:toBe|toEqual|toStrictEqual)\(\1\)|assert_eq!\((\w+), \2\)|assert!\(true\)|expect\(true\)\.toBe\(true\)|assertEqual\((\w+), \3\)"),
 'weak': re.compile(r"toBeDefined\(\)|toBeTruthy\(\)|\.toBeGreaterThan\(0\)|assert!\(!\w+\.is_empty\(\)\)"),
 'mock': re.compile(r"vi\.mock\(|jest\.mock\(|vi\.fn\(|jest\.fn\(|mockResolvedValue|mockReturnValue"),
 'selfimp': re.compile(r"from ['\"]\.\./\.\./(?:🟦️|🟨️)?['\"]"),
}
lines=[l.strip() for l in open(sys.argv[1],encoding='utf-8')]
hdr=None
out=[]
for row in open(sys.argv[2],encoding='utf-8').read().splitlines()[1:]:
    parts=row.split('\t'); path=parts[0]
    full=os.path.join(REPO,path)
    if not os.path.isfile(full) or os.path.getsize(full)>2_000_000: continue
    hits={k:[] for k in rx}
    with open(full,encoding='utf-8',errors='replace') as h:
        for n,line in enumerate(h,1):
            for k,r in rx.items():
                if r.search(line) and len(hits[k])<3: hits[k].append(f"{n}:{line.strip()[:110]}")
                elif r.search(line): hits[k].append('')
    flags={k:len([x for x in v if x]) for k,v in hits.items()}
    if any(flags.values()):
        out.append((path,flags,{k:v for k,v in hits.items() if v and any(v)}))
for p,f,h in out:
    if f['skip'] or f['taut'] or f['weak'] or f['env'] or f['mock']:
        print(p[-95:], {k:v for k,v in f.items() if v})
        for k,v in h.items():
            if k in ('skip','taut','env') or (k=='weak' and f['weak']>0):
                for x in v[:2]:
                    if x: print('    ',k,x)
