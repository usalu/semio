import json,sys
d=json.load(open(sys.argv[1]));o=d['sources'].get('owners',{});l=o.get('ledger') or {}
print('==',sys.argv[1].split('/')[-1],'up',d['uptimeSeconds'],'ledger',l,'closureCounts',d['sources'].get('closureCounts'),'presence',o.get('presence'),'fleet',{k:v for k,v in (o.get('fleet') or {}).items() if v})
print('looks',json.dumps(o.get('looks'))[:600])
p=d['sources'].get('profile')
for i,b in enumerate(p['books']):
    if not b['samples']:continue
    print(' bucket',i,'samples',b['samples'])
    for kind in('self','inclusive'):
        print('  ',kind)
        for r in b[kind][:int(sys.argv[2]) if len(sys.argv)>2 else 18]:print('    %5d %-38s %s:%d'%(r['samples'],r['name'][:38],r['file'][-34:],r['line']))
