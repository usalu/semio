import json,glob,sys,time
end=time.time()+float(sys.argv[1]);need=int(sys.argv[2]) if len(sys.argv)>2 else 20
while time.time()<end:
    for f in glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json'):
        try:d=json.load(open(f))
        except Exception:continue
        o=d['sources'].get('owners') or {}
        if o.get('presence') is not None and (o.get('ledger') or {}).get('done',0)>=need:
            print('ready',f.split('/')[-2]);sys.exit(0)
    time.sleep(3)
print('timeout')
