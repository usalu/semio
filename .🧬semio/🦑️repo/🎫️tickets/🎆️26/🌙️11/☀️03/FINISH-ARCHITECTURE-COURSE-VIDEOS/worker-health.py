import psutil,time
match=('Beat1_DataToInformation','Beat4_PartialModels','🎓️teaching.🧰️interop.🎬️scene.🧪️tests.🐍️')
rows={}
for p in psutil.process_iter(['pid','ppid','cmdline','memory_info','cpu_times']):
 try:
  c=p.info['cmdline'] or []
  if any(x in c for x in match) and p.info['memory_info'].rss>100*2**20:
   t=p.info['cpu_times'];rows[p.pid]=(c[-1],t.user+t.system,p.info['memory_info'].rss/2**30)
 except (psutil.NoSuchProcess,psutil.AccessDenied): pass
print('available_gib',round(psutil.virtual_memory().available/2**30,2))
for pid,(name,t,rss) in rows.items(): print(pid,ascii(name),round(t,1),round(rss,2))
