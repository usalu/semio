import re,sys,collections
for p in open(sys.argv[1],encoding='utf-8').read().split('\n'):
    p=p.strip()
    if not p: continue
    try: lines=open(p,encoding='utf-8').read().split('\n')
    except Exception: continue
    seen=collections.defaultdict(list)
    depth_ind=None
    for i,l in enumerate(lines):
        m=re.match(r'^((?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?)(fn|struct|enum|const|static|type|trait|mod)\s+([A-Za-z_][A-Za-z0-9_]*)',l)
        if m:
            # skip if preceded by cfg attr
            prev=lines[i-1].strip() if i else ''
            if prev.startswith('#[cfg') or prev.startswith('#[test]'): 
                key=(m.group(2),m.group(3),prev)
            else: key=(m.group(2),m.group(3),'')
            ns='type' if m.group(2) in('struct','enum','trait','type','mod') else 'value'
            seen[(ns,m.group(3),key[2])].append(i+1)
    for k,v in seen.items():
        if len(v)>1: print(f"{p.split('norm/')[-1].replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}: {k[1]} defined at lines {v}")
