import json,glob,time,os
SC=os.path.dirname(os.path.abspath(__file__))
log=open(SC+"/series.jsonl","a")
for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json')):
    try:d=json.load(open(f))
    except Exception:continue
    o=d['sources'].get('owners',{}).get('ledger',{});a=d['sources'].get('afterCollect')
    row={'o':f.split('/')[-2],'t':time.strftime('%H:%M:%S'),'up':d['uptimeSeconds'],'heap':d['memory']['heapUsed'],'ext':d['memory']['external'],'rss':d['memory']['rss'],'present':o.get('present'),'done':o.get('done'),'objs':d.get('heap',{}).get('objectCount'),'after':a,'top':[(c['name'],round(c['estimatedBytes']/1048576,1)) for c in d.get('collections',[])[:5]]}
    log.write(json.dumps(row)+"\n")
    print(row['o'],row['t'],'up',row['up'],'heap',row['heap'],'ext',row['ext'],'rss',row['rss'],'present',row['present'],'after',(a or {}).get('heapUsed'),'objs',row['objs'])
