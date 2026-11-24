import json,glob,os,sys
d=sys.argv[1]
for f in sorted(glob.glob(d+'/p*-*.json')):
    if os.path.getsize(f)==0: print(os.path.basename(f),'EMPTY'); continue
    j=json.load(open(f)); ph=[p for p in j['phases'] if p['label'].startswith('run')][0]
    c=j['profiles'][0]['counts']['closure']; calls=j['calls']
    print(f"{os.path.basename(f)[:46]:46} wall {ph['wallMs']/1000:6.1f} cpu {ph['cpuMs']/1000:6.1f} peak {ph['peakMB']:5} der {c['derived']:2} load {c['loaded']:2} stale {c['stale']} fetch {c['fetched']:2} stat {calls.get('p.stat',0):6} wr {calls.get('p.writeFile',0):5} rd {calls.get('readFileSync',0)} err {bool(ph.get('error'))}")
