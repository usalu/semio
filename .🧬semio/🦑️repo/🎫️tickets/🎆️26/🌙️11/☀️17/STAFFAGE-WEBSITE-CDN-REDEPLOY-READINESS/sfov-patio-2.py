import importlib.util, re
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🪑️objects/patio-heater')
for side, x in (('l', 0.165), ('r', -0.19)):
    a = l.shape(d, 50, 'front', f'wheel-{side}')
    text = d.t[a['s']:a['e']]
    indent = re.search(r'\n([ ]*)$', d.t[:a['s']]).group(1)
    t = text.replace(f'"wheel-{side}"', f'"wheel-axle-{side}"')
    t = re.sub(r'"x": [-0-9.]+', f'"x": {x}', t)
    t = re.sub(r'"w": [-0-9.]+', '"w": 0.025', t)
    t = re.sub(r'"y": [-0-9.]+', '"y": 0.0425', t)
    t = re.sub(r'"h": [-0-9.]+', '"h": 0.015', t)
    d.rep({'s': a['e'], 'e': a['e']}, ',\n' + indent + t)
d.commit()
