import json,re,sys
r=json.load(open(sys.argv[1],encoding='utf8'))
cfg=sys.argv[2] if len(sys.argv)>2 else 'test'
seen=set()
for d in r['summary'][cfg]['new_list']:
    f=d['file']; m=re.search(r'__m_[^/]+/(.*)$',f)
    f=m.group(1) if m else f
    parts=[p for p in f.split('/') if p!='..']
    key=(d['code'],d['msg'][:60],parts[-3] if len(parts)>2 else '',d['line'])
    if key in seen: continue
    seen.add(key)
    print(d['code'],d['msg'][:80],'|','/'.join(parts[-3:]),d['line'])
