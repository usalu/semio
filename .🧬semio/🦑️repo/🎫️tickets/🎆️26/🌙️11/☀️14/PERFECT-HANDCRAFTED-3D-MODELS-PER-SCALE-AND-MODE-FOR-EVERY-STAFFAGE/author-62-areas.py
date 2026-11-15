"""Author 62: print per-region outer/exposed areas against areaM2 for one scale. Usage: python3 author-62-areas.py asset scale"""
import sys; sys.path.insert(0, '.')
from author_62_lib import *
A, sc = sys.argv[1], sys.argv[2]
out = run(['volumes-check.ts', A, '--scale', sc, '--product', '--rules', 'volumes-thermal-coverage'])
for l in out.splitlines():
    if 'volumes-thermal-coverage' in l and l[:4] in ('FAIL', 'ok  '):
        o = dict((k, float(v)) for k, v in re.findall(r'info:(outerAreaM2:[\w-]+)=([\d.e+-]+)', l))
        e = dict((k, float(v)) for k, v in re.findall(r'info:(exposedAreaM2:[\w-]+)=([\d.e+-]+)', l))
        dev = re.findall(r'areaDeviation:([\w-]+)=([\d.e+-]+)', l)
        print(l[:4], re.findall(r'(thermal\.\w+=\d+)', l)[:2])
        for r, v in dev:
            print(f'  {r:14s} outer {o.get("outerAreaM2:"+r, 0):8.3f} exposed {e.get("exposedAreaM2:"+r, 0):8.3f} dev {float(v):.3f}')
        print(re.findall(r'areaDeviationTotal=([\d.e+-]+)', l))
