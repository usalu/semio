import json,psutil
rows=[]
for pid in [48900,47000,42304]:
 try:
  p=psutil.Process(pid);rows.append({"pid":pid,"name":p.name(),"birth":p.create_time(),"parent":p.ppid(),"status":p.status(),"cpu":sum(p.cpu_times()[:2]),"read_bytes":p.io_counters().read_bytes,"read_count":p.io_counters().read_count,"rss":p.memory_info().rss,"children":[{"pid":c.pid,"parent":c.ppid(),"name":c.name(),"birth":c.create_time(),"cpu":sum(c.cpu_times()[:2])} for c in p.children(recursive=True)]})
 except (psutil.NoSuchProcess,psutil.AccessDenied) as e:rows.append({"pid":pid,"error":str(e)})
print(json.dumps(rows))
