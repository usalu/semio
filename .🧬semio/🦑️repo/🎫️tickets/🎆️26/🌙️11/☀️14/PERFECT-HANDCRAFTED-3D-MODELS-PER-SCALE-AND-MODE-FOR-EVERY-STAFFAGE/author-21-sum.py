import json,subprocess,sys,glob,os
slug=sys.argv[1]; scales=sys.argv[2:] or ['50','100','200','500']
BUN=glob.glob(os.path.expanduser('~/Library/Caches/semio/runtimes/bun/*/bun'))[0]
for s in scales:
    out=subprocess.run([BUN,'volumes-check.ts','vegetation/'+slug,'--scale',s,'--json'],capture_output=True,text=True).stdout
    d=json.loads(out[out.index('{'):])
    bad=[m for m in d['measurements'] if not m['ok']]
    print('== 1:%s'%s, 'fails',len(bad))
    for m in bad:
        print('  ',m['rule'].replace('volumes-',''),m['name'],round(m['value'],4),'limit',round(m['limit'],4) if m['limit']<1e6 else 'inf',(m.get('note') or '')[:110])
