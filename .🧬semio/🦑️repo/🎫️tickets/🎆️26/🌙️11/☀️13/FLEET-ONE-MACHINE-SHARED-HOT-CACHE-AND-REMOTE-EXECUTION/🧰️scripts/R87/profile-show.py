import json,sys,glob
origins=sys.argv[1:] or [f.split('/')[-2] for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json'))]
for o in origins:
    try:d=json.load(open('/Users/Shared/semio-store/v1/origins/%s/slice-memory.json'%o))
    except Exception as e:print(o,e);continue
    p=d['sources'].get('profile');ow=d['sources'].get('owners',{});l=ow.get('ledger') or {}
    print('==',o,'pid',d['pid'],'up',d['uptimeSeconds'],'done',l.get('done'),'closureCounts',d['sources'].get('closureCounts'),'presence',ow.get('presence'))
    if not p:continue
    for i,b in enumerate(p['books']):
        if not b['samples']:continue
        print(' bucket',i,'samples',b['samples'],'dropped',b['dropped'])
        for kind in('self','inclusive'):
            print('  ',kind)
            for r in b[kind][:14]:print('    %5d %-38s %s:%d'%(r['samples'],r['name'][:38],r['file'][-34:],r['line']))
