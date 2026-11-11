import psutil
print('available_gib', round(psutil.virtual_memory().available / 2**30, 2))
for p in psutil.process_iter(['pid', 'cmdline', 'memory_info']):
 try:
  c = p.info['cmdline'] or []
  if any(x in c for x in ('Beat5_DataTrees', 'Beat1_DataToInformation', 'Beat3_Tower')):
   print(p.pid, c[-1], round(p.info['memory_info'].rss / 2**30, 2))
 except (psutil.NoSuchProcess, psutil.AccessDenied): pass

for p in psutil.process_iter(['pid','ppid','name','cmdline','memory_info']):
 try:
  c=p.info['cmdline'] or []
  if any('unittest' in x.lower() or 'test-v4' in x.lower() for x in c): print('test',p.pid,'parent',p.ppid(),'rss_gib',round(p.info['memory_info'].rss/2**30,2),'name',p.info['name'])
 except (psutil.NoSuchProcess,psutil.AccessDenied): pass
