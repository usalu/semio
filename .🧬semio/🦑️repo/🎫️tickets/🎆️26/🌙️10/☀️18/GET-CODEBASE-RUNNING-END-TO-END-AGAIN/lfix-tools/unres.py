"""🔎 Unresolved local names of one oracle log per file: usage unres.py <err> <path-fragment>... ; prints file-tail:line | message for resolve-class codes outside macro/async_test noise."""
import json,sys,re
path,frags=sys.argv[1],sys.argv[2:]
RES={"E0412","E0425","E0422","E0423","E0432","E0433","E0405","E0531","E0532","E0574","E0576","E0599","E0609","E0061","E0308","E0560","E0026","E0107"}
seen=set()
for line in open(path,encoding='utf-8'):
    line=line.strip()
    if not line.startswith('{'): continue
    try: d=json.loads(line)
    except Exception: continue
    if d.get('level')!='error': continue
    c=(d.get('code') or {}).get('code') or ''
    for s in d.get('spans',[]):
        if s.get('is_primary') and any(f in s['file_name'] for f in frags):
            m=d['message']
            if 'async_test' in m or 'serde_json' in m and 'json' in m: continue
            key=(s['file_name'],s['line_start'],m)
            if key in seen: continue
            seen.add(key)
            print(f"{c} {'/'.join(s['file_name'].split('/')[-3:-1])}:{s['line_start']}: {m[:150]}")
