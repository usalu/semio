import psutil
for p in psutil.process_iter(['pid','cmdline']):
 try:
  if any('excel128-oracle.py' in x for x in (p.info['cmdline'] or [])): print(p.pid)
 except (psutil.NoSuchProcess,psutil.AccessDenied): pass
