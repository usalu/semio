import psutil
print('available_gib',round(psutil.virtual_memory().available/2**30,2))
rows=[]
for p in psutil.process_iter(['pid','ppid','name','memory_info','cmdline']):
 try:
  m=p.info['memory_info']; c=p.info['cmdline'] or []
  if m and m.rss>300*2**20: rows.append((round(m.rss/2**30,2),p.pid,p.ppid(),p.info['name'],c[-1] if c else ''))
 except (psutil.NoSuchProcess,psutil.AccessDenied): pass
for x in sorted(rows,reverse=True)[:12]: print(ascii(x))
