import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4s2 as P
RX = re.compile(r'#\[(?:\w+::)*(?:async_)?test\]((?:\s*#\[[^\]]*(?:\][^\]]*)*?\])*)\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)')
tag, prefix, script = sys.argv[1], sys.argv[2], sys.argv[3]
sp = importlib.import_module('specs.' + tag).SPEC
live = os.path.join(P.REPO, os.path.dirname(sp['file']))
where = {}
root_names = [m.group(2) for m in RX.finditer(open(os.path.join(live, '🦀️.rs'), encoding='utf-8').read())]
for n in root_names:
    where[n] = ''
for c in sp['children']:
    t = open(os.path.join(live, c['folder'], '🦀️.rs'), encoding='utf-8').read()
    for m in RX.finditer(t):
        assert m.group(2) not in where, m.group(2)
        where[m.group(2)] = c['mod'] + '::'
text = open(script, encoding='utf-8').read()
pat = re.compile(re.escape(prefix) + r'(?:[a-z_0-9]+::)?([A-Za-z_0-9]+)"')
miss = []
def sub(m):
    n = m.group(1)
    if n not in where:
        miss.append(n)
        return m.group(0)
    return prefix + where[n] + n + '"'
new = pat.sub(sub, text)
print('rewritten', sum(1 for _ in pat.finditer(text)), 'missing', miss)
if os.environ.get('APPLY'):
    open(script, 'w', encoding='utf-8').write(new)
    print('written')
