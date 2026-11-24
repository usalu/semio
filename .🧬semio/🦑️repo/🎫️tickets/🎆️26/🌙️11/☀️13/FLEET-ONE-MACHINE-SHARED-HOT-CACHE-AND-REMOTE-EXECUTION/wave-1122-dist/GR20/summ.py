import json,sys,re,glob,os
for f in sorted(sys.argv[1:]):
    try: d=json.load(open(f))
    except Exception as e: print(f,"no json"); continue
    err=open(f.replace('.json','.err')).read()
    m=re.search(r'([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys',err); rss=re.search(r'(\d+)\s+maximum resident',err)
    run=[p for p in d['phases'] if p['label'].startswith('run:')]
    c=d['profiles'][0]['counts'] if d['profiles'] else {}
    dr=re.search(r"\[GR19\] drain (\{.*\})",err); drain=json.loads(dr.group(1)) if dr else {}
    print(os.path.basename(f),"drainMs",drain.get("wallMs"),"drainCpuMs",drain.get("cpuMs"),"queuedBefore",drain.get("queuedBefore"),"wall",m and m.group(1),"user+sys",m and round(float(m.group(2))+float(m.group(3)),1),"rssMB",rss and round(int(rss.group(1))/1048576),"runCpuMs",run and run[0]['cpuMs'],"err",run and run[0].get('error'),"closure",c.get('closure'),"lane",c.get('lane'),"writes",(d.get('calls') or {}).get('p.writeFile'),"stat",(d.get('calls') or {}).get('p.stat'))
