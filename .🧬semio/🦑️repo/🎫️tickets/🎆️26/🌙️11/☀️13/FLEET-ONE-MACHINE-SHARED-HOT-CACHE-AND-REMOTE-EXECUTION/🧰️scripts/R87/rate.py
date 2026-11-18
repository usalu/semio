import json,glob,time,os
SC=os.path.dirname(os.path.abspath(__file__));P=SC+"/rate.json"
prev=json.load(open(P)) if os.path.exists(P) else {}
cur={}
for f in sorted(glob.glob('/Users/Shared/semio-store/v1/origins/g1-m*/slice-memory.json')):
    try:d=json.load(open(f))
    except Exception:continue
    o=d['sources'].get('owners',{});l=o.get('ledger',{});n=f.split('/')[-2]
    done=l.get('done') or 0;up=d['uptimeSeconds'];cur[n]=(d['atMs']/1000,done,up,d['pid'])
    r=''
    if n in prev and prev[n][3]==cur[n][3] and up>prev[n][2]:r='  owners/s %.2f'%((done-prev[n][1])/(up-prev[n][2]))
    print(n,'pid',d['pid'],'up',up,'heap',d['memory']['heapUsed'],'rss',d['memory']['rss'],'done',done,'MB/owner %.1f'%((d['memory']['heapUsed']-450)/max(done,1)),r)
json.dump(cur,open(P,'w'))
