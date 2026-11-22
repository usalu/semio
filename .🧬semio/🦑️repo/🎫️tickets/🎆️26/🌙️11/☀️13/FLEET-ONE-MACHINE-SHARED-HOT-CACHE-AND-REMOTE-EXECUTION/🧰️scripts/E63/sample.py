import json,sys,time
out=open(sys.argv[1],'a')
end=time.time()+float(sys.argv[2])
last=None
while time.time()<end:
    try:
        d=json.load(open('/Users/ueli/Documents/semio/.cache/f/slice-memory.json'))
        s=d.get('sources',{});inv=s.get('inventory',{})
        row=[time.strftime('%T'),d['pid'],int(d['uptimeSeconds']),d['phase']['name'][:90],json.dumps(inv.get('active'))[:160],json.dumps(inv.get('activeOwner'))[:150],json.dumps(inv.get('completed'))[:300]]
        k=json.dumps(row[1:])
        if k!=last: out.write(' | '.join(map(str,row))+'\n');out.flush();last=k
    except Exception as e: pass
    time.sleep(5)
