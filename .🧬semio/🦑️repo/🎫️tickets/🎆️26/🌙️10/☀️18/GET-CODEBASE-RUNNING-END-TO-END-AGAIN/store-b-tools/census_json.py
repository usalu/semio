import json,sys,collections,re
by=collections.defaultdict(list)
for line in open(sys.argv[1],errors='replace'):
    if not line.startswith('{'):continue
    try:j=json.loads(line)
    except:continue
    if j.get('reason')!='compiler-message':continue
    m=j['message']
    if m.get('level')!='error':continue
    pid=j.get('package_id','');name=re.search(r'(semio-[a-z0-9-]+|#[a-z0-9_-]+)',pid)
    pk=pid.split('#')[-1].split('@')[0] if '#' in pid else pid
    by[pk].append(m)
only=sys.argv[2] if len(sys.argv)>2 else ''
for pk,ms in sorted(by.items()):
    if only and only not in pk:continue
    print(pk,len(ms))
    if only:
        seen=set()
        for m in ms:
            sp=[s for s in m['spans'] if s['is_primary']]
            loc=f"{sp[0]['file_name'][-110:]}:{sp[0]['line_start']}:{sp[0]['column_start']}" if sp else ''
            key=(m['message'],loc)
            if key in seen:continue
            seen.add(key);print('  ',loc,m['message'][:200])
