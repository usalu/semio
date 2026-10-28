"""🧭 Streams a product run log and prints phase transitions against the 10 s heartbeat clock."""
import re, sys, resource
hb = re.compile(r'^\[repository:command\] running elapsedMs=(\d+)')
act = re.compile(r'"command":"([^"]+)".*?"resource":"([^"]+)"')
plain = re.compile(r'^\[script\] ([^ ]+) · (.*)$')
t = 0.0; last = None; n = 0
for line in open(sys.argv[1], errors='replace'):
    n += 1
    if n % 20000 == 0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 2**20: sys.exit('rss')
    m = hb.match(line)
    if m: t = int(m.group(1)) / 1000; continue
    key = None
    m = act.search(line)
    if m:
        res = m.group(2)
        res = re.sub(r':\d+(:\d+)*(:%.*)?$', '', res) if res.startswith('build:graph-phase') else res.split(':')[0]
        key = f'{m.group(1)} {res}'
    elif line.startswith('[script] {') and '"command":"' in line:
        key = re.search(r'"command":"([^"]+)"', line).group(1)
    elif not line.startswith('[script]') and line.strip():
        key = 'OUT ' + line.strip()[:110]
    if key and key != last:
        print(f'{t:7.0f} {key}'); last = key
