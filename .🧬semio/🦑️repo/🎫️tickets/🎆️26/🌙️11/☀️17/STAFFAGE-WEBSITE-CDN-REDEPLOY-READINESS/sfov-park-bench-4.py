import importlib.util, json, re
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/park-bench')
for side, bone, x in (('l', 'end-left', 0.058), ('r', 'end-right', 1.618)):
    a = l.shape(d, 50, 'plan', f'{side}-armrest')
    text = d.t[a['s']:a['e']]
    indent = re.search(r'\n([ ]*)$', d.t[:a['s']]).group(1)
    t = text.replace(f'"{side}-armrest"', f'"{side}-foot"').replace('"z": 5.2', '"z": 2')
    t = re.sub(r'"x": [-0-9.]+', f'"x": {x}', t)
    t = re.sub(r'"w": [-0-9.]+', '"w": 0.124', t)
    t = re.sub(r'"h": [-0-9.]+', '"h": 0.55', t)
    t = re.sub(r'"r": [-0-9.]+', '"r": 0.01', t)
    t = re.sub(r'"y": [-0-9.]+', '"y": -0.3', t)
    d.rep({'s': a['s'], 'e': a['s']}, t + ',\n' + indent)
d.commit()
