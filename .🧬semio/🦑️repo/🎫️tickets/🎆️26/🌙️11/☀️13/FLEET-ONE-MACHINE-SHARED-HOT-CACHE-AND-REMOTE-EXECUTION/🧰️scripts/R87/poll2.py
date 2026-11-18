import json,glob,time,os,sys
D=os.path.dirname(os.path.abspath(__file__));out=open(D+"/series.jsonl","a")
open(D+"/poll.pid","w").write(str(os.getpid()));os.makedirs(D+"/reports",exist_ok=True)
end=time.time()+float(sys.argv[1]);last={}
while time.time()<end:
    for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json')):
        try:d=json.load(open(f))
        except Exception:continue
        n=f.split('/')[-2];o=d['sources'].get('owners') or {};l=o.get('ledger') or {}
        row={"o":n,"t":d['atMs']/1000,"pid":d['pid'],"up":d['uptimeSeconds'],"done":l.get('done'),"total":l.get('total'),"heap":d['memory']['heapUsed'],"cpu":o.get('cpuSeconds'),"phases":(o.get('looks') or {}).get('phases'),"count":(o.get('looks') or {}).get('count'),"closureCounts":d['sources'].get('closureCounts'),"presence":o.get('presence'),"layers":d['sources'].get('layers'),"fleet":o.get('fleet')}
        out.write(json.dumps(row)+"\n")
        p=d['sources'].get('profile')
        if p and sum(b['samples'] for b in p['books'])>0:
            key=(n,d['pid'],d['atMs']//120000)
            if key not in last:
                last[key]=1;json.dump(d,open(D+"/reports/%s-%d-%d.json"%(n,d['pid'],int(d['uptimeSeconds'])),"w"))
    out.flush();time.sleep(30)
