"""📋️ Round 3 tram: digest of `volumes-check.ts` output read from stdin (failing measurements, thermal areas against the declared values)."""
import re, sys
for line in sys.stdin:
    line = line.rstrip()
    m = re.match(r'(ok|FAIL)\s+(\S+)\s+1:(\d+)\s+\[\w\]\s+(.*)', line)
    if not m:
        if line.startswith(('FAIL', 'PASS')):
            print(line)
        continue
    st, rule, sc, rest = m.groups()
    if rule == 'volumes-thermal-coverage':
        outer = dict(re.findall(r'info:outerAreaM2:([\w-]+)=([\d.e-]+)', rest))
        dev = dict(re.findall(r'!?areaDeviation:([\w-]+)=([\d.e-]+)', rest))
        bad = set(re.findall(r'!areaDeviation:([\w-]+)', rest))
        print(st, rule, sc, ' '.join(f"{'!' if k in bad else ''}{k}={float(v):.2f}({float(dev.get(k, 0)):.2f})" for k, v in outer.items()), ' '.join(re.findall(r'thermal\.\S+', rest)))
    elif st == 'FAIL' or '--all' in sys.argv:
        print(st, rule, sc, ' '.join(t[:90] for t in rest.split() if t.startswith('!') or '--all' in sys.argv))
