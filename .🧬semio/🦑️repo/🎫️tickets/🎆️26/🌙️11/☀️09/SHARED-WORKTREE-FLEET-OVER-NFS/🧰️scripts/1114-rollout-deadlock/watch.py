import json,time,sys,glob,os
want=sys.argv[1]; end=time.time()+float(sys.argv[2])
seen={}
while time.time()<end:
    for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-*/attest.json')):
        o=f.split('/')[-2]
        try: a=json.load(open(f))
        except Exception: continue
        st=(a['code'][:12],(a.get('pending') or {}).get('code','-')[:12],int(time.time()-a['wallMs']/1000))
        if o not in seen or seen[o][0]!=st[0] or seen[o][1]!=st[1]:
            print(time.strftime('%H:%M:%S'),o,'code',st[0],'pending',st[1],'age',st[2],flush=True)
        seen[o]=st
    time.sleep(5)
