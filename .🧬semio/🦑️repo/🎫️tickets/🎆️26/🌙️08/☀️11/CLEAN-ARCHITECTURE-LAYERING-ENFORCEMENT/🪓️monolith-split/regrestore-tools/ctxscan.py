import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from touched import *
tot = 0
for sid in SPECS:
    t = touched(sid)
    flagged = []
    for p in t:
        lines = read(os.path.join(ROOT, p)).split('\n')
        for n, ln in enumerate(lines):
            m = MARK.match(ln.rstrip('\r'))
            if not m:
                continue
            prev = lines[n - 1].strip() if n > 0 else ''
            nxt = lines[n + 1].strip() if n + 1 < len(lines) else ''
            isopen = not m.group(2)
            bad = None
            if prev.startswith(('#[', '///', '/**', '/*')) or prev.endswith('*/') or (prev.startswith('//') and not MARK.match(lines[n - 1])):
                bad = 'after attr/comment'
            if prev.endswith((',', '(', '=>', '=', '&&', '||', '+', '?', ':')) and not prev.startswith('//'):
                bad = (bad or '') + ' after continuation'
            if isopen and nxt.startswith(('}', ')', ']')):
                bad = (bad or '') + ' open before closer'
            if not isopen and nxt.startswith(('.', 'else')):
                bad = (bad or '') + ' close before continuation'
            if bad:
                flagged.append((p[-70:], n + 1, ln.strip()[:40], bad, prev[:50], nxt[:40]))
    tot += len(flagged)
    print(sid, 'touched', len(t), 'flagged', len(flagged))
    for f in flagged[:8]:
        print('   ', f)
print('total flagged', tot)
