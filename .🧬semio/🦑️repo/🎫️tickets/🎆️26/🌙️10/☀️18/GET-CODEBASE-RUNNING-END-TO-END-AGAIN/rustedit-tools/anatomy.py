"""🧭 Streams a Bazel test.log / action stderr and prints a timeline: per 10 s heartbeat of the action child, the distinct phases seen."""
import re, sys, resource
from collections import Counter

beat = re.compile(r'^\[@semio-tech/[^\]]+:test:[a-z]+\] running elapsedMs=(\d+)')
label = re.compile(r'^\[script\] ([a-z:-]+ · [A-Za-z ,]+?)(?::| \d|$)')
cmd = re.compile(r'"command":"([^"]+)"')
res = re.compile(r'"resource":"([^"]{0,80})')
cargo = re.compile(r'^\s+(Compiling|Fresh|Finished|Running|Executable|Dirty|Blocking)\b(.{0,90})')
other = re.compile(r'^\[(cargo[^\]]*|compiler[^\]]*|unit[^\]]*|lease-order|capacity|native[^\]]*)\] (.{0,140})')

def flush(t, seen, cargos, extras):
    parts = ', '.join(f'{k}×{v}' for k, v in seen.most_common(8))
    print(f'{t:>6}s | {parts}')
    for c in cargos[:6]: print(f'        cargo: {c}')
    if len(cargos) > 6: print(f'        cargo: … {len(cargos)} lines')
    for e in extras[:4]: print(f'        {e}')

t = 0; seen = Counter(); cargos = []; extras = []; warns = 0
for line in open(sys.argv[1], errors='replace'):
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 2**20: sys.exit('rss')
    m = beat.match(line)
    if m:
        flush(t, seen, cargos, extras); t = int(m.group(1)) // 1000; seen = Counter(); cargos = []; extras = []; continue
    if line.startswith('warning:'): warns += 1; continue
    m = label.match(line)
    if m: seen[m.group(1)] += 1; continue
    m = cmd.search(line)
    if m:
        r = res.search(line); seen[m.group(1) + (' @' + r.group(1).split('/')[-1][:30] if r else '')] += 1; continue
    m = cargo.match(line)
    if m: cargos.append(m.group(1) + m.group(2)); continue
    m = other.match(line)
    if m and 'elapsedMs' not in line: extras.append(line.strip()[:200])
flush(t, seen, cargos, extras)
print('warnings', warns)
