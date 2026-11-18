import json,glob,sys,time
for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json')):
    try:d=json.load(open(f))
    except Exception as e:print(f,e);continue
    o=d['sources'].get('owners',{}).get('ledger',{})
    print(f.split('/')[-2],'up',d['uptimeSeconds'],'heapUsed',d['memory']['heapUsed'],'present',o.get('present'),'col' ,'collections' in d)
    for c in d.get('collections',[])[:int(sys.argv[1]) if len(sys.argv)>1 else 8]:print('    %-60s %9d %10.1f MB'%(c['name'],c['entries'],c['estimatedBytes']/1048576))
