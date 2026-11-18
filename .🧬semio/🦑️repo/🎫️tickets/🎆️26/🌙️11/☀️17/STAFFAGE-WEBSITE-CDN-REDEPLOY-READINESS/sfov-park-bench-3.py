import importlib.util, json, re
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/park-bench')
n = l.shape(d, 50, 'front', 'seat-chamfer')
text = d.t[n['s']:n['e']]
indent = re.search(r'\n([ ]*)$', d.t[:n['s']]).group(1)
pts = s.walk(n, ['points'])
ptxt = d.t[pts['s']:pts['e']]
parts = []
for k in range(4):
    x0 = -0.3 + 0.13 * k + 0.012; x1 = -0.3 + 0.13 * k + 0.1 - 0.012
    t = text.replace('"seat-chamfer"', f'"seat-chamfer-{k+1}"')
    new = re.sub(r'(\[\s*)-0\.288(\s*,\s*)0\.441(\s*\]\s*,\s*\[\s*)0\.178(\s*,\s*)0\.441', lambda m: f'{m.group(1)}{x0:.3f}{m.group(2)}0.441{m.group(3)}{x1:.3f}{m.group(4)}0.441', t)
    assert new != t
    parts.append(new)
d.rep(n, (',\n' + indent).join(parts))
d.commit()
