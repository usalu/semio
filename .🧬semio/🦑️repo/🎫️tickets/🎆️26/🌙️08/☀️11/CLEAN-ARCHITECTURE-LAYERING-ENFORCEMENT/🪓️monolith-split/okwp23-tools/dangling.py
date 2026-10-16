# 🪝 Dangling-attribute scan: an outer attribute (`#[...]`) followed by a blank line, another attribute then a blank line, or end of file means a dropped item left its attributes behind and they now gate the next item. Compares the live tree with the pristine snapshot so pre-existing oddities are not reported.
# usage: dangling.py
import os, re, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE)
import unionlib as U
R = U.R; SCR = U.SCR
def hits(text):
    lines = text.split('\n'); out = []
    for i, l in enumerate(lines):
        s = l.strip()
        if re.match(r'#\[[^\]]*\]$', s) and not s.startswith('#[doc'):
            j = i + 1
            while j < len(lines) and re.match(r'\s*#\[[^\]]*\]\s*$', lines[j]): j += 1
            if j >= len(lines) or lines[j].strip() == '': out.append((i + 1, s))
    return out
bad = 0
for rel in U.touched():
    if not rel.endswith('.rs'): continue
    p = os.path.join(R, rel)
    if not os.path.exists(p): continue
    now = hits(open(p, encoding='utf8').read())
    op = os.path.join(SCR, 'orig', rel)
    was = {h[1] for h in hits(open(op, encoding='utf8').read())} if os.path.exists(op) else set()
    new = [h for h in now if h[1] not in was or True]
    old_n = len(hits(open(op, encoding='utf8').read())) if os.path.exists(op) else 0
    if len(now) > old_n:
        bad += 1; print(rel[-95:], 'now', len(now), 'was', old_n, [h for h in now][:4])
print('files with new dangling attributes:', bad)
